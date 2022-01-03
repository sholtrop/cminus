use crate::{
    error::ICodeError, flow_graph::FlowGraph, ic_info::ICLineNumber, icode::IntermediateCode,
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
    if matches!(opt_level, OptLevel::Pre | OptLevel::Post) {
        preprocess(tree, table)
    }

    let func_ids = table.get_function_ids();
    let mut visitor = IVisitor::new(table);
    for id in func_ids {
        let func = tree
            .get_root(&id)
            .unwrap_or_else(|| panic!("Function with id {} not found", id));
        visitor.visit_function(func, id);
    }
    let mut icode = visitor.result();
    if matches!(opt_level, OptLevel::Post | OptLevel::Both) {
        postprocess(&mut icode, table);
    }
    let graph = FlowGraph::new(table, &icode);
    for (l, _) in (&icode).into_iter() {
        log::trace!("{}. is reachable: {}", l, graph.is_reachable(&l));
    }
    log::trace!("Live at 3:\n{:#?}", graph.get_live_at(&ICLineNumber(3)));

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
