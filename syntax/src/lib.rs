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

use crate::builder::SyntaxBuilder;

pub struct SyntaxResult {
    tree: SyntaxTree,
    symbol_table: SymbolTable,
}

/// Take an input string and generate a [SyntaxTree] and [SymbolTable] for it.
/// They are returned in the form of a [SyntaxResult].
pub fn generate(input: &str) -> Result<SyntaxResult, Box<dyn Error>> {
    let parse_tree = lexical::parse(input)?;
    let mut builder = SyntaxBuilder::new();
    builder.parsetree_to_syntaxtree(parse_tree)?;
    todo!("Implement")
}
