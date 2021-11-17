mod builder;
mod error;
mod id;
mod node;
mod scope;
mod symbol;
mod symbol_table;
mod syntax_tree;
mod visitor;

use general::logging;
use syntax::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logger();
    let input = std::fs::read_to_string(
        "/home/sholtrop/development/rust/cminus/syntax/tests/node/correct/statementlist_funccall.c",
    )?;
    generate(&input).and(Ok(()))
}
