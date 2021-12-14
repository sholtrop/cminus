use clap::clap_app;
use general::logging;
use log::LevelFilter;
use pest_ascii_tree::into_ascii_tree;

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
    let input_file = matches.value_of("INPUT").unwrap();
    let input = std::fs::read_to_string(input_file).unwrap();
    let result = lexical::parse(&input).map_err(|e| {
        log::error!("{}", e);
        e
    })?;
    let tree = into_ascii_tree(result)?;
    log::info!("\n{}", tree);
    log::info!("Parsed successfully");
    Ok(())
}
