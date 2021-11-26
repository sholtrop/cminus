pub mod lexical_test;
pub mod lib;
pub mod syntax_test;

use general::logging::init_logger;
use std::io;

fn main() -> io::Result<()> {
    init_logger();
    // lexical_test::run()?;
    syntax_test::run()?;
    Ok(())
}
