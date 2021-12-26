use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use error::ICodeError;
use flow_graph::FlowGraph;
use ic_generator::Intermediate;
use ic_generator::OptLevel;
use syntax::{SymbolTable, SyntaxAnalysisResult, SyntaxTree};

pub mod error;
pub mod flow_graph;
pub mod ic_generator;
pub mod ic_info;
pub mod intermediate_code;
pub mod ioperand;
pub mod ioperator;
pub mod istatement;
pub mod ivisitor;

pub fn generate(
    tree: &SyntaxTree,
    symbol_table: &mut SymbolTable,
    opt: OptLevel,
) -> Result<Intermediate, ICodeError> {
    ic_generator::generate(tree, symbol_table, opt)
}

pub fn generate_from_str(input: &str, opt: OptLevel) -> Result<Intermediate, ICodeError> {
    let SyntaxAnalysisResult {
        mut symbol_table,
        tree,
        ..
    } = syntax::generate(input).unwrap();
    generate(&tree, &mut symbol_table, opt)
}

pub fn save_cfg(filename: &str, graph: &FlowGraph) {
    let mut dot = Command::new("dot")
        .arg("-Tpng")
        .arg("-o")
        .arg(filename)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            panic!(
                "Could not create {}. Is graphviz installed on your system?\n{}",
                filename, e
            )
        });
    dot.stdin
        .as_mut()
        .unwrap()
        .write_all(graph.to_string().as_bytes())
        .unwrap();
    log::info!(
        "Saved control flow graph to {} with entrypoint {}",
        filename,
        graph.entry()
    );
}
