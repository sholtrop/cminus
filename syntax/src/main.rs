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

use clap::clap_app;
use general::logging;
use log::LevelFilter;
use syntax::SyntaxAnalysisResult;

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
    let input = matches.value_of("INPUT").unwrap();
    let input = std::fs::read_to_string(input)?;
    let SyntaxAnalysisResult {
        errors,
        warnings,
        symbol_table,
        tree,
    } = syntax::generate(&input)?;
    let has_errors = !errors.is_empty();
    if !has_errors || show_partial {
        log::info!("\n{}", symbol_table);
        log::info!("\n{}", tree);
    }
    if has_errors {
        syntax::display_errors(&errors);
    }
    syntax::display_warnings(&warnings);
    Ok(())
}
