pub mod allocator;
pub mod assembly;
pub mod code_generator;
pub mod emitter;
pub mod output;
pub mod register;

use clap::clap_app;
use general::logging::init_logger_from_env;
use machine_code::compile_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce x86 assembly code for the given input C-minus file")
        (@arg INPUT: +required "Sets the input")
        (@arg OUTPUT: -o +takes_value "Sets the output")
    )
    .get_matches();
    init_logger_from_env();
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT");
    compile_file(input, output)?;
    Ok(())
}
