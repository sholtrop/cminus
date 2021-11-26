mod builder;
mod error;
mod id;
mod node;
mod scope;
mod symbol;
mod symbol_table;
mod syntax_tree;
mod tree_walker;
mod visitor;

use lexical::ParseTree;
use std::error::Error;
use tree_walker::TreeWalker;
use visitor::{SyntaxResult, Visitor};

/// Take an input [ParseTree] and generate a [SyntaxResult] for it containing the syntax tree + symbol table.
pub fn generate(input: ParseTree) -> Result<SyntaxResult, Box<dyn Error>> {
    let mut tree_walker = TreeWalker::new();
    let mut visitor = Visitor::new();
    tree_walker.construct_syntax_tree(input, &mut visitor)?;
    Ok(visitor.result())
}
