pub mod assembly;
pub mod code_generator;
pub mod emitter;
pub mod output;
pub mod reg_alloc;
pub mod register;

use clap::clap_app;
use general::logging::init_logger_from_env;
use intermediate_code::ic_generator::OptLevel;
use machine_code::compile_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Produce x86 assembly code for the given input C-minus file")
        (@arg INPUT: +required "Sets the input")
        (@arg OUTPUT: -o +takes_value "Sets the output")
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

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT");
    compile_file(input, output, opt_level)?;
    log::info!("Compilation successful");
    Ok(())
}
