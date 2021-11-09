use std::io;

use general::logging::init_logger;
use tests::collect_tests_in_path;
use tests::run_test;
use tests::TestStage;
use tests::TestStats;

fn run_tests(stats: &mut TestStats, stage: TestStage) -> io::Result<()> {
    for path in stage.get_paths() {
        collect_tests_in_path(path, stats)?
            .into_iter()
            .for_each(|test| {
                stats.total += 1;
                if run_test(test, stage.into()).is_ok() {
                    stats.success += 1;
                }
            });
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    init_logger();
    run_tests(&mut stats, TestStage::Lexical)?;
    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
