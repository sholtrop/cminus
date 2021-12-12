use crate::{
    error::ICodeError, flow_graph::FlowGraph, intermediate_code::IntermediateCode,
    ivisitor::IVisitor,
};
use std::fmt;
use syntax::{SymbolTable, SyntaxTree};

pub struct Intermediate {
    icode: IntermediateCode,
    graph: FlowGraph,
}

fn preprocess(tree: &SyntaxTree, table: &mut SymbolTable) {}

pub fn generate(tree: &SyntaxTree, table: &mut SymbolTable) -> Result<Intermediate, ICodeError> {
    let visitor = IVisitor::new(table);
    Err(ICodeError::from("Not implemented yet"))
}

fn postprocess(icode: &mut IntermediateCode, table: &mut SymbolTable) {}

impl fmt::Display for Intermediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Flow graph - Not implemented yet")?;
        writeln!(f, "#### Intermediate code ####")?;
        writeln!(f, "\n{}", self.icode)
    }
}
