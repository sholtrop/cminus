use error::ICodeError;
use ic_generator::Intermediate;
use syntax::{SymbolTable, SyntaxTree};

pub mod error;
pub mod flow_graph;
pub mod ic_generator;
pub mod id;
pub mod intermediate_code;
pub mod ioperand;
pub mod ioperator;
pub mod istatement;
pub mod ivisitor;

pub enum OptLevel {
    None,
    Pre,
    Post,
    Both,
}

pub fn generate(
    tree: &SyntaxTree,
    symbol_table: &mut SymbolTable,
    opt: OptLevel,
) -> Result<Intermediate, ICodeError> {
    ic_generator::generate(tree, symbol_table, opt)
}
