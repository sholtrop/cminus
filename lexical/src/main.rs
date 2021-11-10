mod lib;

use general::logging;
use lib::parse;

use pest_ascii_tree::into_ascii_tree;

const INPUT: &str = "
int main(void) {
    unsigned a = 1;
    writeunsigned(a);
    unsigned int b = -1; /* equals 2^32-1 */
    writeunsigned(b);
    return 0;
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
