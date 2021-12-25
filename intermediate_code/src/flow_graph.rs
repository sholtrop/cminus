use crate::{
    ic_info::{ICInfo, ICLineNumber},
    intermediate_code::IntermediateCode,
};
use id_arena::Arena;
use std::{borrow::Cow, fmt};
use syntax::SymbolTable;

#[derive(Clone)]
pub struct BasicBlock {
    start: ICLineNumber,
    end: ICLineNumber,
    incoming: Vec<id_arena::Id<BasicBlock>>,
    outgoing: Vec<id_arena::Id<BasicBlock>>,
}

impl BasicBlock {
    pub fn new(start: ICLineNumber, end: ICLineNumber) -> Self {
        Self {
            start,
            end,
            incoming: vec![],
            outgoing: vec![],
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
        let info = FlowGraph::icode_to_info(icode);
        let mut iter = info.leaders.iter();
        let mut entry = None;
        let mut prev_line = iter.next().unwrap();
        log::trace!("before loop");
        for curr_line in iter {
            log::trace!("{} {}", prev_line, curr_line);
            graph.alloc(BasicBlock::new(*prev_line, *curr_line - 1));
            prev_line = curr_line;
        }
        graph.alloc(BasicBlock::new(*prev_line, icode.n_statements().into()));
        // for (id, block) in graph.iter_mut() {}
        Self { graph, entry }
    }

    fn icode_to_info(icode: &IntermediateCode) -> ICInfo {
        let mut info = ICInfo::new();
        let mut statements = icode.into_iter().peekable();
        while let Some((line, stmt)) = statements.next() {
            log::trace!("{}", line);
            if stmt.is_label() {
                info.leaders.insert(line);
                let id = stmt.label_id();
                info.labels.insert(id, line);
            } else if stmt.is_func() {
                info.leaders.insert(line);
                let id = stmt.label_id();
                info.funcs.insert(line, id);
            } else if stmt.is_jump() && statements.peek().is_some() {
                info.leaders.insert(line + 1);
            } else if stmt.is_call() {
                let id = stmt.label_id();
                info.add_call(id, line);
                if statements.peek().is_some() {
                    info.leaders.insert(line + 1);
                }
            }
        }
        log::trace!("Info:\n{:#?}", info);
        info
    }

    fn get_outgoing_edges(
        &self,
        block: BasicBlock,
        icode: &IntermediateCode,
    ) -> Vec<id_arena::Id<BasicBlock>> {
        let mut edges = vec![];
        for s in icode.into_iter().skip(block.start.0).take(block.end.0) {}
        edges
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
}

impl<'a> dot::GraphWalk<'a, Vertex<'a>, Edge> for &FlowGraph {
    fn nodes(&'a self) -> dot::Nodes<'a, Vertex> {
        let v: Vec<Vertex> = self.graph.iter().collect();
        Cow::Owned(v)
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        let mut v = Vec::<Edge>::with_capacity(self.graph.len());
        for (id, node) in self.graph.iter() {
            for neighbor in node.incoming.clone() {
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
