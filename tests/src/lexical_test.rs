use std::io;

use tests::{collect_tests_in_path, run_single_test};

use crate::lib::TestStats;

const PROGRAM_TEST_PATH: &str = "tests/testfiles/general/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/general/units";
const LEXICAL_TEST_PATH: &str = "tests/testfiles/lexical";

pub fn test_function(input: &str) -> Result<(), &str> {
    lexical::parse(input).and(Ok(())).or(Err("error"))
}

pub fn run() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    let unit_tests = collect_tests_in_path(UNIT_TEST_PATH)?.into_iter();
    let program_tests = collect_tests_in_path(PROGRAM_TEST_PATH)?.into_iter();
    let lex_tests = collect_tests_in_path(LEXICAL_TEST_PATH)?.into_iter();

    for test in unit_tests.chain(program_tests).chain(lex_tests) {
        stats.total += 1;
        if run_single_test(test, test_function).is_ok() {
            stats.success += 1;
        }
    }

    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
