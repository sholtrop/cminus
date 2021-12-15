pub mod error;
pub mod flow_graph;
pub mod ic_generator;
pub mod id;
pub mod intermediate_code;
pub mod ioperand;
pub mod ioperator;
pub mod istatement;
pub mod ivisitor;

use crate::error::ICodeError;
use ::intermediate_code::OptLevel;
use clap::clap_app;
use general::logging::init_logger_from_env;
use syntax::SyntaxAnalysisResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce intermediate code for the given input C-minus file")
        (@arg annotate: -a --annotate "Also print the annotated intermediate code")
        (@arg INPUT: +required "Sets the input")
    )
    .get_matches();
    // let level = if matches.is_present("verbose") {
    //     LevelFilter::Trace
    // } else {
    //     LevelFilter::Info
    // };
    init_logger_from_env();
    let annotate = matches.is_present("annotate");
    let input = matches.value_of("INPUT").unwrap();
    let input = std::fs::read_to_string(input)?;
    let SyntaxAnalysisResult {
        errors,
        mut symbol_table,
        tree,
        warnings,
    } = syntax::generate(&input)?;
    let has_errors = !errors.is_empty();
    if has_errors {
        syntax::display_errors(&errors);
        return Err(Box::new(ICodeError(format!(
            "{} syntax error{} encountered",
            errors.len(),
            if errors.len() == 1 { "" } else { "s" }
        ))));
    }
    syntax::display_warnings(&warnings);
    let ic = ic_generator::generate(&tree, &mut symbol_table, OptLevel::None);
    log::info!("\n{}", symbol_table);
    log::info!("\n{}", tree);
    match ic {
        Ok(ic) => {
            log::info!("\n{}", ic);
            if annotate {
                log::info!(
                    "\nAnnotated:\n{}",
                    symbol_table.annotate_icode(ic.to_string())
                );
            }
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
