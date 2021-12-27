use crate::{
    ic_info::{ICInfo, ICLineNumber},
    icode::IntermediateCode,
};
use id_arena::Arena;
use std::{borrow::Cow, collections::HashMap, fmt};
use syntax::SymbolTable;

#[derive(Clone)]
pub struct BasicBlock {
    start: ICLineNumber,
    end: ICLineNumber,
    incoming: Vec<id_arena::Id<BasicBlock>>,
    outgoing: Vec<id_arena::Id<BasicBlock>>,
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
        incoming: Vec<id_arena::Id<BasicBlock>>,
        outgoing: Vec<id_arena::Id<BasicBlock>>,
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

pub struct FlowGraph {
    graph: Arena<BasicBlock>,
    entry: Option<id_arena::Id<BasicBlock>>,
}

impl FlowGraph {
    pub fn new(table: &SymbolTable, icode: &IntermediateCode) -> Self {
        let mut graph = Arena::new();
        let info = ICInfo::from(icode);
        let mut iter = info.leaders.iter();
        let mut entry = None;
        let mut prev_leader = iter.next().unwrap();
        let mut leader_to_block = HashMap::new();
        for curr_leader in iter {
            log::trace!("leader: {}", curr_leader);
            let block = BasicBlock::new(*prev_leader, *curr_leader - 1);
            let id = graph.alloc(block);
            leader_to_block.insert(*prev_leader, id);
            if FlowGraph::line_is_main(table, prev_leader, icode) {
                entry = Some(id);
                graph.get_mut(id).unwrap().is_entry = true;
            }
            prev_leader = curr_leader;
        }
        {
            let last = graph.alloc(BasicBlock::new(
                *prev_leader,
                (icode.n_statements() - 1).into(),
            ));
            leader_to_block.insert(*prev_leader, last);
            if FlowGraph::line_is_main(table, prev_leader, icode) {
                entry = Some(last);
                graph.get_mut(last).unwrap().is_entry = true;
            }
        }
        for block_id in leader_to_block.values() {
            let out;
            {
                let block = graph.get_mut(*block_id).unwrap();
                out = FlowGraph::get_outgoing_edges(block, icode, &info, &leader_to_block);
                for b in out.iter() {
                    block.outgoing.push(*b);
                }
            }
            for b in out {
                let block = graph.get_mut(b).unwrap();
                block.incoming.push(*block_id);
            }
        }
        FlowGraph::add_calls(&mut graph, &leader_to_block, &info);
        Self { graph, entry }
    }

    pub fn entry(&self) -> &BasicBlock {
        self.graph
            .get(self.entry.expect("No entrypoint set on CFG"))
            .unwrap()
    }

    /// Get the [BasicBlock] ids for all the outgoing edges of the given basic block except for calls
    fn get_outgoing_edges(
        block: &BasicBlock,
        icode: &IntermediateCode,
        info: &ICInfo,
        leaders: &HashMap<ICLineNumber, id_arena::Id<BasicBlock>>,
    ) -> Vec<id_arena::Id<BasicBlock>> {
        let mut out = vec![];
        let last_stmt = icode.get_statement(block.end);
        if last_stmt.is_unconditional_jump() {
            let label = last_stmt.label_id();
            log::trace!("uncond - labelid: {}", label);
            let leader = info.labels.get(&label).unwrap();
            let block = leaders.get(leader).unwrap();
            out.push(*block);
        } else if last_stmt.is_conditional_jump() {
            let leader_if = block.end + 1;
            let block = leaders.get(&leader_if).unwrap();
            out.push(*block);
            let label = last_stmt.label_id();
            log::trace!("cond - labelid: {}", label);
            let leader_else = info.labels.get(&label).unwrap();
            let block = leaders.get(leader_else).unwrap();
            out.push(*block);
        } else if last_stmt.is_call() {
            log::trace!("call");
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
        leader_to_block: &HashMap<ICLineNumber, id_arena::Id<BasicBlock>>,
        info: &ICInfo,
    ) {
        log::trace!("Returns: {:#?}", info.returns);
        for (id, calls) in &info.calls {
            if let Some(returns) = info.returns.get(id) {
                // Requires complex scoping to keep the borrow checker happy
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

type Vertex<'a> = (id_arena::Id<BasicBlock>, &'a BasicBlock);
type Edge = (id_arena::Id<BasicBlock>, id_arena::Id<BasicBlock>);

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
