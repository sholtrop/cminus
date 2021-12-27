pub mod builder;
pub mod error;
pub mod id;
pub mod node;
pub mod scope;
pub mod symbol;
pub mod symbol_table;
pub mod syntax_tree;
pub mod tree_walker;
pub mod visitor;

use std::error::Error;
use tree_walker::TreeWalker;
use visitor::Visitor;

pub use error::*;
pub use id::*;
pub use node::*;
pub use symbol::*;
pub use symbol_table::*;
pub use syntax_tree::*;
pub use visitor::*;

/// Take an input string and generate a [SyntaxResult] for it containing the syntax tree + symbol table.
pub fn generate(input: &str) -> Result<SyntaxAnalysisResult, Box<dyn Error>> {
    let parse_tree = lexical::parse(input)?;
    let mut tree_walker = TreeWalker::new();
    let mut visitor = Visitor::new();
    tree_walker.construct_syntax_tree(parse_tree, &mut visitor)?;
    let syntax_res = visitor.result();
    Ok(syntax_res)
}

pub fn display_errors(errors: &[(SyntaxBuilderError, usize)]) {
    for (err, line) in errors {
        log::error!("Line {}: {}", line, err);
    }
}

pub fn display_warnings(warnings: &[(SyntaxBuilderWarning, usize)]) {
    for (warning, line) in warnings {
        log::warn!("Line {}: {}", line, warning);
    }
}
