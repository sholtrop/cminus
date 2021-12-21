use crate::{
    error::ICodeError, flow_graph::FlowGraph, intermediate_code::IntermediateCode,
    ivisitor::IVisitor,
};
use std::fmt;
use syntax::{SymbolTable, SyntaxTree};

pub enum OptLevel {
    None,
    Pre,
    Post,
    Both,
}

pub struct Intermediate {
    pub icode: IntermediateCode,
    pub graph: FlowGraph,
}

fn preprocess(tree: &SyntaxTree, table: &mut SymbolTable) {}

pub fn generate(
    tree: &SyntaxTree,
    table: &mut SymbolTable,
    opt_level: OptLevel,
) -> Result<Intermediate, ICodeError> {
    match opt_level {
        OptLevel::Pre | OptLevel::Both => preprocess(tree, table),
        _ => {}
    };

    let func_ids = table.get_function_ids();
    let mut visitor = IVisitor::new(table);
    for id in func_ids {
        let func = tree.get_root(&id).expect("Function not found");
        visitor.visit_function(func, id);
    }
    let mut icode = visitor.result();
    match opt_level {
        OptLevel::Post | OptLevel::Both => postprocess(&mut icode, table),
        _ => {}
    };
    let graph = FlowGraph::new(table, &icode);
    Ok(Intermediate { icode, graph })
}

fn postprocess(icode: &mut IntermediateCode, table: &mut SymbolTable) {}

impl fmt::Display for Intermediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Flow graph - Not implemented yet")?;
        writeln!(f, "#### Intermediate code ####")?;
        writeln!(f, "{}", self.icode)
    }
}
