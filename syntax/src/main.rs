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
use syntax::{NodeType, SyntaxAnalysisResult, SyntaxNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce an abstract syntax tree for the given input C-minus file")
        (@arg verbose: -v --verbose "Print debug information")
        (@arg show_partial: -s --show_partial "Shows the partial syntax tree built up until this point, even in case of an error")
        (@arg INPUT: +required "Sets the input") 
    )
    .get_matches();
    let level = if matches.is_present("verbose") {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    let show_partial = matches.is_present("show_partial");
    logging::init_logger(level);
    let input = std::fs::read_to_string("test.c")?;
    let parse_tree = lexical::parse(&input)?;
    let SyntaxAnalysisResult {
        errors,
        warnings,
        symbol_table,
        tree,
    } = syntax::generate(parse_tree)?;
    let has_errors = !errors.is_empty();
    if !has_errors || show_partial {
        log::info!("\n{}", symbol_table);
        log::info!("\n{}", tree);
    }
    for (warning, line) in warnings {
        log::warn!("Line {}: {}", line, warning);
    }
    if has_errors {
        for (err, line) in errors {
            log::error!("Line {}: {}", line, err);
        }
    }

    Ok(())
}
