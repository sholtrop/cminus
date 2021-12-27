pub mod allocator;
pub mod code_emitter;
pub mod code_generator;
pub mod register;

use clap::clap_app;
use general::logging::init_logger_from_env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce x86 assembly code for the given input C-minus file")
        (@arg INPUT: +required "Sets the input")
    )
    .get_matches();
    init_logger_from_env();
    Ok(())
}
