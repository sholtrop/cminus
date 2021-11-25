mod lib;

use general::logging;
use lib::parse;

use pest_ascii_tree::into_ascii_tree;

const INPUT: &str = "
int x;
int main(void) {
    x = 1 + 2 - 3 * 4 % (5 / a);
}

";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logger();
    let result = parse(INPUT).map_err(|e| {
        log::error!("{}", e);
        e
    })?;
    let tree = into_ascii_tree(result)?;
    log::info!("{}", tree);
    log::info!("Parsed successfully");
    Ok(())
}
