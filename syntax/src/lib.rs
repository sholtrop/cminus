mod builder;
mod error;
mod id;
mod node;
mod scope;
mod symbol;
mod symbol_table;
mod syntax_tree;

use std::error::Error;
use symbol_table::SymbolTable;
use syntax_tree::SyntaxTree;

pub struct SyntaxResult {
    tree: SyntaxTree,
    symbol_table: SymbolTable,
}

/// Take an input string and generate a SyntaxTree and SymbolTable for it.
pub fn generate(input: &str) -> Result<SyntaxResult, Box<dyn Error>> {
    let parse_tree = lexical::parse(input)?;
    todo!("Implement")
}
