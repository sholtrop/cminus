use error::ICodeError;
use ic_generator::Intermediate;
use ic_generator::OptLevel;
use syntax::{SymbolTable, SyntaxAnalysisResult, SyntaxTree};

pub mod error;
pub mod flow_graph;
pub mod ic_generator;
pub mod ic_info;
pub mod icode;
mod icode_optimization;
pub mod ioperand;
pub mod ioperator;
pub mod istatement;
pub mod ivisitor;
mod syntax_tree_optimization;

pub fn generate(
    tree: &mut SyntaxTree,
    symbol_table: &mut SymbolTable,
    opt: OptLevel,
) -> Result<Intermediate, ICodeError> {
    ic_generator::generate(tree, symbol_table, opt)
}

pub fn generate_from_str(input: &str, opt: OptLevel) -> Result<Intermediate, ICodeError> {
    let SyntaxAnalysisResult {
        mut symbol_table,
        mut tree,
        ..
    } = syntax::generate(input).unwrap();
    generate(&mut tree, &mut symbol_table, opt)
}
