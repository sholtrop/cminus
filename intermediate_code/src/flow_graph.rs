use crate::{
    ic_info::{ICInfo, ICLineNumber},
    icode::IntermediateCode,
    ioperand::IOperand,
    ioperator::IOperator,
};
use id_arena::Arena;
use itertools::Itertools;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    iter::FromIterator,
};
use syntax::{SymbolId, SymbolTable};

#[derive(Clone)]
pub struct BasicBlock {
    start: ICLineNumber,
    end: ICLineNumber,
    incoming: Vec<BasicBlockId>,
    outgoing: Vec<BasicBlockId>,
    is_entry: bool,
}

impl BasicBlock {
    pub fn new(start: ICLineNumber, end: ICLineNumber) -> Self {
        Self {
            start,
            end,
            incoming: vec![],
            outgoing: vec![],
            is_entry: false,
        }
    }

    pub fn new_with(
        start: ICLineNumber,
        end: ICLineNumber,
        incoming: Vec<BasicBlockId>,
        outgoing: Vec<BasicBlockId>,
    ) -> Self {
        Self {
            start,
            end,
            incoming,
            outgoing,
            is_entry: false,
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start != self.end {
            write!(f, "B{}_{}", self.start, self.end)
        } else {
            write!(f, "B{}", self.start)
        }
    }
}

#[derive(Default, Debug)]
pub struct Liveness {
    pub live_out: HashMap<ICLineNumber, HashSet<SymbolId>>, // variables live after line
    pub live_in: HashMap<ICLineNumber, HashSet<SymbolId>>,  // variables live before line
    pub def: HashMap<ICLineNumber, HashSet<SymbolId>>,      // variables defined by line
    pub used: HashMap<ICLineNumber, HashSet<SymbolId>>,     // variables used by line
}

pub type BasicBlockId = id_arena::Id<BasicBlock>;
type Blockmap = HashMap<ICLineNumber, BasicBlockId>;

pub struct FlowGraph {
    graph: Arena<BasicBlock>,
    entry: BasicBlockId,
    reachable: HashSet<BasicBlockId>,
    block_map: HashMap<ICLineNumber, BasicBlockId>,
    liveness: Liveness,
}

impl FlowGraph {
    pub fn new(table: &SymbolTable, icode: &IntermediateCode) -> Self {
        let info = ICInfo::from(icode);
        let (entry, graph, block_map) = FlowGraph::build_graph(icode, table, &info);
        let reachable = FlowGraph::determine_reachable(entry, &graph);
        let liveness = FlowGraph::compute_liveness(icode, table, &graph);
        Self {
            graph,
            entry,
            block_map,
            liveness,
            reachable,
        }
    }

    pub fn entry(&self) -> &BasicBlock {
        self.graph.get(self.entry).unwrap()
    }

    pub fn is_reachable(&self, line: &ICLineNumber) -> bool {
        let block = self.block_map.get(line).unwrap();
        self.reachable.contains(block)
    }

    pub fn is_live_at(&self, line: &ICLineNumber, sym: &SymbolId) -> bool {
        match self.liveness.live_out.get(line) {
            Some(live) => live.contains(sym),
            None => false,
        }
    }

    /// Get all variables which are live at given line
    pub fn get_live_at(&self, line: &ICLineNumber) -> HashSet<SymbolId> {
        let mut live = match self.liveness.live_out.get(line) {
            Some(l) => l.clone(),
            None => HashSet::new(),
        };
        if let Some(l) = self.liveness.live_in.get(line) {
            live.extend(l.iter())
        }
        live
    }

    /// Get all variables which are live after the given line
    pub fn get_live_out_at(&self, line: &ICLineNumber) -> HashSet<SymbolId> {
        match self.liveness.live_out.get(line) {
            Some(l) => l.clone(),
            None => HashSet::new(),
        }
    }

    /// Returns a three-tuple of the entry to the graph, the graph itself and the blockmap
    fn build_graph(
        icode: &IntermediateCode,
        table: &SymbolTable,
        info: &ICInfo,
    ) -> (BasicBlockId, Arena<BasicBlock>, Blockmap) {
        let mut graph = Arena::new();
        let mut iter = info.leaders.iter();
        let mut entry = None;
        let mut prev_leader = iter.next().unwrap();
        let mut leader_to_block = HashMap::new();
        let mut block_map = HashMap::new();
        for curr_leader in iter {
            log::trace!("leader: {}", curr_leader);
            let block = BasicBlock::new(*prev_leader, *curr_leader - 1);
            let id = graph.alloc(block);
            for line in prev_leader.0..curr_leader.0 {
                log::trace!("line: {}", line);
                block_map.insert(ICLineNumber(line), id);
            }
            leader_to_block.insert(*prev_leader, id);
            if FlowGraph::line_is_main(table, prev_leader, icode) {
                entry = Some(id);
                graph.get_mut(id).unwrap().is_entry = true;
            }
            prev_leader = curr_leader;
        }
        {
            let last_line = (icode.n_statements() - 1).into();
            let last = graph.alloc(BasicBlock::new(*prev_leader, last_line));
            leader_to_block.insert(*prev_leader, last);
            if FlowGraph::line_is_main(table, prev_leader, icode) {
                entry = Some(last);
                graph.get_mut(last).unwrap().is_entry = true;
            }
            for line in prev_leader.0..(last_line + 1).0 {
                log::trace!("line: {}", line);
                block_map.insert(ICLineNumber(line), last);
            }
        }
        for block_id in leader_to_block.values() {
            let out;
            {
                let block = graph.get_mut(*block_id).unwrap();
                out = FlowGraph::get_outgoing_edges(block, icode, info, &leader_to_block);
                for b in out.iter() {
                    block.outgoing.push(*b);
                }
            }
            for b in out {
                let block = graph.get_mut(b).unwrap();
                block.incoming.push(*block_id);
            }
        }
        FlowGraph::add_calls(&mut graph, &leader_to_block, info);
        (
            entry.expect("No entry found in FlowGraph"),
            graph,
            block_map,
        )
    }

    fn compute_liveness_block(
        block: &BasicBlock,
        icode: &IntermediateCode,
        liveness: &mut Liveness,
    ) {
        let Liveness {
            live_in,
            live_out,
            def,
            used,
        } = liveness;
        let mut worklist: VecDeque<ICLineNumber> = icode
            .get_statements(block.start, block.end)
            .iter()
            .zip(block.start.0..block.end.0)
            .map(|(_, ln)| ICLineNumber(ln))
            .rev()
            .collect();

        while let Some(ln) = worklist.pop_front() {
            let succ_in = if ln.0 <= block.end.0 {
                Some(live_in.get(&(ln + 1)).unwrap().clone())
            } else {
                None
            };
            let used = used.get_mut(&ln).unwrap();
            let def = def.get_mut(&ln).unwrap();
            let live_out = live_out.get_mut(&ln).unwrap();
            let live_in = live_in.get_mut(&ln).unwrap();
            let old_live_in = live_in.clone();
            if let Some(successor) = succ_in {
                *live_out = successor.clone();
            }
            *live_in = used
                .union(&live_out.difference(def).copied().collect())
                .copied()
                .collect();
            if old_live_in != *live_in {
                for pred in (block.start.0..ln.0).rev() {
                    worklist.push_back(ICLineNumber(pred));
                }
            }
        }
    }

    fn compute_liveness(
        icode: &IntermediateCode,
        table: &SymbolTable,
        graph: &Arena<BasicBlock>,
    ) -> Liveness {
        let mut liveness = Liveness::default();
        let globals = table.get_globals().keys().copied().collect_vec();
        for (l, stmt) in icode {
            let mut def = HashSet::new();
            if stmt.operator == IOperator::Assign {
                log::trace!("{}", stmt);
                def.insert(stmt.ret_target.as_ref().unwrap().id());
            }
            liveness.def.insert(l, def);

            let mut used = HashSet::new();
            if let Some(IOperand::Symbol { id, .. }) = stmt.operand1.as_ref() {
                used.insert(*id);
            }
            if let Some(IOperand::Symbol { id, .. }) = stmt.operand2.as_ref() {
                used.insert(*id);
            }
            liveness.used.insert(l, used);
            liveness
                .live_out
                .insert(l, HashSet::from_iter(globals.clone()));
            liveness.live_in.insert(l, HashSet::new());
        }

        for (_, block) in graph.iter() {
            FlowGraph::compute_liveness_block(block, icode, &mut liveness);
        }
        liveness
    }

    fn determine_reachable(
        entry: BasicBlockId,
        graph: &Arena<BasicBlock>,
    ) -> HashSet<BasicBlockId> {
        graph_walk(graph, entry).collect()
    }

    /// Get the [BasicBlock] ids for all the outgoing edges of the given basic block except for calls
    fn get_outgoing_edges(
        block: &BasicBlock,
        icode: &IntermediateCode,
        info: &ICInfo,
        leaders: &HashMap<ICLineNumber, BasicBlockId>,
    ) -> Vec<BasicBlockId> {
        let mut out = vec![];
        let last_stmt = icode.get_statement(block.end);
        if last_stmt.is_unconditional_jump() {
            let label = last_stmt.label_id();
            let leader = info.labels.get(&label).unwrap();
            let block = leaders.get(leader).unwrap();
            out.push(*block);
        } else if last_stmt.is_conditional_jump() {
            let leader_if = block.end + 1;
            let block = leaders.get(&leader_if).unwrap();
            out.push(*block);
            let label = last_stmt.label_id();
            let leader_else = info.labels.get(&label).unwrap();
            let block = leaders.get(leader_else).unwrap();
            out.push(*block);
        } else if last_stmt.is_non_builtin_call() {
            let func_id = last_stmt.label_id();
            let line = info.funcs.get(&func_id).unwrap();
            let block = leaders.get(line).unwrap();
            out.push(*block);
        } else if last_stmt.is_return() {
            // Function doesn't know where it will return to. This is handled in [FlowGraph::add_calls].
        } else if let Some(next_block_id) = leaders.get(&(block.end + 1)) {
            // The next line is a label.
            // Control flow will therefore naturally go from current block to the next.
            let next_line = icode.get_statement(block.end + 1);
            if next_line.is_label() {
                out.push(*next_block_id);
            }
        }
        out
    }

    fn line_is_main(table: &SymbolTable, line: &ICLineNumber, icode: &IntermediateCode) -> bool {
        let stmt = icode.get_statement(*line);
        if !stmt.is_func() {
            return false;
        }
        let sym_id = icode.get_statement(*line).label_id();
        sym_id == table.get_main_id()
    }

    fn add_calls(
        graph: &mut Arena<BasicBlock>,
        leader_to_block: &HashMap<ICLineNumber, BasicBlockId>,
        info: &ICInfo,
    ) {
        log::trace!("Returns: {:#?}", info.returns);
        for (id, calls) in &info.calls {
            if let Some(returns) = info.returns.get(id) {
                for call in calls {
                    let after_call_bid = *leader_to_block.get(&(*call + 1)).unwrap();
                    for ret in returns {
                        let ret_bid = {
                            let (ret_bid, ret_block) =
                                graph.iter_mut().find(|(_, b)| b.end == *ret).unwrap();
                            log::trace!("{} returns to {}", ret, *call + 1);
                            ret_block.outgoing.push(after_call_bid);
                            ret_bid
                        };
                        let call_block = graph.get_mut(after_call_bid).unwrap();
                        call_block.incoming.push(ret_bid);
                    }
                }
            }
        }
    }
}

fn graph_walk(graph: &Arena<BasicBlock>, entry: BasicBlockId) -> GraphIter {
    GraphIter {
        graph,
        to_visit: vec![entry],
        visited: HashSet::new(),
    }
}

pub struct GraphIter<'a> {
    graph: &'a Arena<BasicBlock>,
    to_visit: Vec<BasicBlockId>,
    visited: HashSet<BasicBlockId>,
}

impl<'a> Iterator for GraphIter<'a> {
    type Item = BasicBlockId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.to_visit.pop() {
            let block = self.graph.get(next).unwrap();
            for id in &block.outgoing {
                if !self.visited.contains(id) {
                    self.to_visit.push(*id);
                }
            }
            self.visited.insert(next);
            Some(next)
        } else {
            None
        }
    }
}

type Vertex<'a> = (BasicBlockId, &'a BasicBlock);
type Edge = (BasicBlockId, BasicBlockId);

impl<'a> dot::Labeller<'a, Vertex<'a>, Edge> for &FlowGraph {
    fn graph_id(&'a self) -> dot::Id {
        dot::Id::new("Control_Flow_Graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> dot::Id {
        let (_, node) = n;
        dot::Id::new(node.to_string()).unwrap()
    }

    fn node_shape(&'a self, n: &Vertex<'a>) -> Option<dot::LabelText<'a>> {
        if n.1.is_entry {
            Some(dot::LabelText::LabelStr("box".into()))
        } else {
            None
        }
    }
}

impl<'a> dot::GraphWalk<'a, Vertex<'a>, Edge> for &FlowGraph {
    fn nodes(&'a self) -> dot::Nodes<'a, Vertex> {
        let v: Vec<Vertex> = self.graph.iter().collect();
        Cow::Owned(v)
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        let mut v = Vec::<Edge>::with_capacity(self.graph.len());
        for (id, node) in self.graph.iter() {
            for neighbor in node.outgoing.clone() {
                v.push((id, neighbor));
            }
        }
        Cow::Owned(v)
    }

    fn source(&'a self, edge: &Edge) -> Vertex<'a> {
        let edge = edge.0;
        let vert = self.graph.get(edge).unwrap();
        (edge, vert)
    }

    fn target(&'a self, edge: &Edge) -> Vertex<'a> {
        let edge = edge.1;
        let vert = self.graph.get(edge).unwrap();
        (edge, vert)
    }
}

impl fmt::Display for FlowGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = vec![];
        dot::render(&self, &mut buff).unwrap();
        let graph_str = String::from_utf8(buff).unwrap();
        write!(f, "{}", graph_str)
    }
}
