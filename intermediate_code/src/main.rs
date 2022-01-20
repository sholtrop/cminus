pub mod error;
pub mod flow_graph;
pub mod ic_generator;
pub mod ic_info;
pub mod icode;
mod icode_optimization;
pub mod ioperand;
pub mod ioperator;
pub mod istatement;
pub mod ivisitor;
mod syntax_tree_optimization;

use crate::error::ICodeError;
use crate::flow_graph::FlowGraph;
use crate::ic_generator::{Intermediate, OptLevel};
use clap::clap_app;
use general::logging::init_logger_from_env;
use std::io::Write;
use std::process::{Command, Stdio};
use syntax::SyntaxAnalysisResult;

fn save_cfg(filename: &str, graph: &FlowGraph) {
    let mut dot = Command::new("dot")
        .arg("-Tpng")
        .arg("-o")
        .arg(filename)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            panic!(
                "Could not create {}. Is graphviz installed on your system?\n{}",
                filename, e
            )
        });
    dot.stdin
        .as_mut()
        .unwrap()
        .write_all(graph.to_string().as_bytes())
        .unwrap();
    log::info!(
        "Saved control flow graph to {} with entrypoint {}",
        filename,
        graph.entry()
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce intermediate code for the given input C-minus file")
        (@arg annotate: -a --annotate "Also print the annotated intermediate code")
        (@arg flowgraph: +takes_value -g --flowgraph  "Save the control flow graph in .png format to the provided file. Requires the Graphviz library (`dot`).")
        (@arg INPUT: +required "Sets the input")
        (@arg OPTIMIZE: -O +takes_value "Optimize compiler output. Takes a value between 0 and 3 (inclusive).")
    )
    .get_matches();
    init_logger_from_env();
    let opt_level = match matches.value_of("OPTIMIZE") {
        Some("1") => OptLevel::Pre,
        Some("2") => OptLevel::Post,
        Some("3") => OptLevel::Both,
        None | Some("0") => OptLevel::None,
        _ => unreachable!(),
    };
    log::info!("Using `{:?}` optimization", opt_level);
    let annotate = matches.is_present("annotate");
    let graph_filename = matches.value_of("flowgraph");
    let input = matches.value_of("INPUT").unwrap();
    let input = std::fs::read_to_string(input)?;
    let SyntaxAnalysisResult {
        errors,
        mut symbol_table,
        mut tree,
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
    let ic = ic_generator::generate(&mut tree, &mut symbol_table, opt_level);
    log::info!("\n{}", symbol_table);
    log::info!("\n{}", tree);
    match ic {
        Ok(Intermediate { ref graph, icode }) => {
            if annotate {
                log::info!(
                    "\nAnnotated:\n{}",
                    symbol_table.annotate_icode(icode.to_string())
                );
            } else {
                log::info!("\n{}", icode);
            }
            if let Some(filename) = graph_filename {
                save_cfg(filename, graph);
            }
            Ok(())
        }

        Err(e) => Err(Box::new(e)),
    }
}
