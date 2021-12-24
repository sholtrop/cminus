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
    others: Vec<id_arena::Id<BasicBlock>>,
}

impl BasicBlock {
    pub fn new(start: ICLineNumber, end: ICLineNumber) -> Self {
        Self {
            start,
            end,
            others: vec![],
        }
    }

    pub fn new_with(
        start: ICLineNumber,
        end: ICLineNumber,
        others: Vec<id_arena::Id<BasicBlock>>,
    ) -> Self {
        Self { start, end, others }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s{}_e{}", self.start, self.end)
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
        Self { graph, entry: None }
    }

    fn icode_to_info(icode: &IntermediateCode) -> ICInfo {
        let mut info = ICInfo::new();
        let mut statements = icode.into_iter().peekable();
        while let Some((line, stmt)) = statements.next() {
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
            for neighbor in node.others.clone() {
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
