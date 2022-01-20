use crate::{
    error::ICodeError, flow_graph::FlowGraph, icode::IntermediateCode, icode_optimization,
    ivisitor::IVisitor, syntax_tree_optimization,
};
use std::fmt;
use syntax::{SymbolTable, SyntaxTree};

#[derive(Debug)]
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

fn preprocess(tree: &mut SyntaxTree, table: &mut SymbolTable) {
    syntax_tree_optimization::fold_constants(tree);
}

fn postprocess(
    icode: &mut IntermediateCode,
    table: &mut SymbolTable,
    flowgraph: &FlowGraph,
) -> FlowGraph {
    // TODO:
    //  - consolidate returns
    //    i.e. Don't copy return code but jump to a single return at the end where applicable

    icode_optimization::eliminate_dead_code(icode, flowgraph, table)
}

pub fn generate(
    tree: &mut SyntaxTree,
    table: &mut SymbolTable,
    opt_level: OptLevel,
) -> Result<Intermediate, ICodeError> {
    if matches!(opt_level, OptLevel::Pre | OptLevel::Both) {
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
    let mut graph = FlowGraph::new(table, &icode);
    if matches!(opt_level, OptLevel::Post | OptLevel::Both) {
        graph = postprocess(&mut icode, table, &graph);
    }
    for (l, _) in (&icode).into_iter() {
        log::trace!("{}. is reachable: {}", l, graph.is_reachable(&l));
    }

    Ok(Intermediate { icode, graph })
}

impl fmt::Display for Intermediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#### Intermediate code ####")?;
        writeln!(f, "{}", self.icode)
    }
}
