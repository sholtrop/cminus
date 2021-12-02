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

use clap::clap_app;
use general::logging;
use log::LevelFilter;
use syntax::{NodeType, SyntaxNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce an abstract syntax tree for the given input C-minus file")
        (@arg INPUT: +required "Sets the input")
        (@arg verbose: -v --verbose "Print debug information")
    )
    .get_matches();
    let level = if matches.is_present("verbose") {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    logging::init_logger(level);
    let input = std::fs::read_to_string("test.c")?;
    let parse_tree = lexical::parse(&input)?;
    let res = syntax::generate(parse_tree)?;
    log::info!("\n{}", res.symbol_table);
    log::info!("\n{}", res.tree);
    for (_, func) in res.tree.functions {
        for node in func.tree.unwrap().preorder() {
            if let SyntaxNode::Constant {
                node_type, value, ..
            } = node
            {
                if *node_type == NodeType::Error {
                    log::error!("Error: {}", value.to_string())
                }
            }
        }
    }
    Ok(())
}
