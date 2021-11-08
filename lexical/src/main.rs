mod lib;
use general::logging;
use lib::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logger();
    parse()?;
    log::info!("Parsed successfully");
    Ok(())
}
