use syntax::SymbolTable;

use crate::{flow_graph::FlowGraph, icode::IntermediateCode};

/// Eliminate dead code and return a new flowgraph that reflects the new icode.
pub fn eliminate_dead_code(
    icode: &mut IntermediateCode,
    graph: &FlowGraph,
    table: &SymbolTable,
) -> FlowGraph {
    log::debug!("Eliminating dead code");
    let mut unreachable = vec![];
    for (l, _) in &*icode {
        if !graph.is_reachable(&l) {
            unreachable.push(l);
        }
    }
    log::debug!(
        "{} line(s) of dead code will be eliminated",
        unreachable.len()
    );
    log::trace!("BEFORE DCE:\n{}", icode);
    for l in unreachable {
        icode.remove_statement(l);
    }
    icode.filter_none();
    log::trace!("AFTER DCE:\n{}", icode);

    FlowGraph::new(table, icode)
}
