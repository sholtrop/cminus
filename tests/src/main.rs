pub mod lexical_test;
pub mod lib;
pub mod syntax_test;

use clap::clap_app;
use general::logging::init_logger;
use log::LevelFilter;
use std::io;

fn main() -> io::Result<()> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce an abstract syntax tree for the given input C-minus file")
        (@arg TESTS: +required "Sets the test(s) to run. One or more of `lexical`, `syntax`")
        (@arg verbose: -v --verbose "Print debug information")
    )
    .get_matches();
    let level = if matches.is_present("verbose") {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    init_logger(level);
    for test in matches.values_of("TESTS").unwrap() {
        if test == "lexical" {
            lexical_test::run()?;
        } else if test == "syntax" {
            syntax_test::run()?;
        } else {
            log::error!("No such test {}", test);
        }
    }
    Ok(())
}
