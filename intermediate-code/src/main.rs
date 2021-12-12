mod error;
mod flow_graph;
mod ic_generator;
mod id;
mod intermediate_code;
mod ioperand;
mod ioperator;
mod istatement;
mod ivisitor;

use crate::error::ICodeError;
use clap::clap_app;
use general::logging::init_logger;
use log::LevelFilter;
use syntax::SyntaxAnalysisResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce intermediate code for the given input C-minus file")
        (@arg verbose: -v --verbose "Print debug information")
        (@arg INPUT: +required "Sets the input")
    )
    .get_matches();
    let level = if matches.is_present("verbose") {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    init_logger(level);
    let input = matches.value_of("INPUT").unwrap();
    let SyntaxAnalysisResult {
        errors,
        mut symbol_table,
        tree,
        warnings,
    } = syntax::generate(input)?;
    let has_errors = !errors.is_empty();
    if has_errors {
        syntax::display_errors(&errors);
        return Err(Box::new(ICodeError::from("Syntax error(s) encountered")));
    }
    syntax::display_warnings(&warnings);
    let ic = ic_generator::generate(&tree, &mut symbol_table)?;
    log::info!("{}", ic);
    Ok(())
}
