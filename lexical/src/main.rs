mod lib;

use general::logging;
use lib::parse;

use pest_ascii_tree::into_ascii_tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logger();
    let result = parse(
        "
    int main(void) {
        func(1, a, 3);
    }",
    )?;
    let tree = into_ascii_tree(result)?;
    log::info!("{}", tree);
    log::info!("Parsed successfully");
    Ok(())
}
