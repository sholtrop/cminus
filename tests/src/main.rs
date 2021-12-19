pub mod intermediate_code_test;
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
        (@arg TESTS: +required "Sets the test(s) to run. One or more of `lexical`, `syntax`, `intermediate`")
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
        match test {
            "lexical" => lexical_test::run()?,
            "syntax" => syntax_test::run()?,
            "intermediate" => intermediate_code_test::run()?,
            _ => log::error!("No such test {}", test),
        }
    }
    Ok(())
}
