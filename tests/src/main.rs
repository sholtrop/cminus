use std::io;

use general::logging::init_logger;
use tests::collect_tests_in_path;
use tests::TestStats;

const PROGRAM_TEST_PATH: &str = "tests/testfiles/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/units";

fn unit_tests(stats: &mut TestStats) -> io::Result<()> {
    collect_tests_in_path(UNIT_TEST_PATH, stats)?;
    Ok(())
}

fn program_tests(stats: &mut TestStats) -> io::Result<()> {
    collect_tests_in_path(PROGRAM_TEST_PATH, stats)?;
    Ok(())
}

fn specific_tests(stats: &mut TestStats) -> io::Result<()> {
    todo!("Implement");
}

fn main() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    init_logger();
    unit_tests(&mut stats)?;
    program_tests(&mut stats)?;
    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
