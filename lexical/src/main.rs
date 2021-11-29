mod lib;

use std::env;

use clap::clap_app;
use general::logging;
use lib::parse;

use log::LevelFilter;
use pest_ascii_tree::into_ascii_tree;

const INPUT: &str = "
int x;
int main(void) {
    x = 1 + 2 - 3 * 4 % (5 / a);
}

";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce a concrete syntax tree for the given input C-minus file")
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
    let result = parse(INPUT).map_err(|e| {
        log::error!("{}", e);
        e
    })?;
    let tree = into_ascii_tree(result)?;
    log::info!("{}", tree);
    log::info!("Parsed successfully");
    Ok(())
}
