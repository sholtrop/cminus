pub mod lexical_test;
pub mod lib;

use general::logging::init_logger;
use std::io;

fn main() -> io::Result<()> {
    init_logger();
    lexical_test::run()?;
    Ok(())
}
