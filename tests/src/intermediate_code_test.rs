use std::io;
use tests::{collect_tests_in_path, run_single_test, Expectation, TestStats};

const PROGRAM_TEST_PATH: &str = "tests/testfiles/general/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/general/units";
const SYNTAX_TEST_PATH: &str = "tests/testfiles/syntax";

pub fn test_function(input: &str) -> Result<(), &str> {
    intermediate_code::generate_from_str(input, intermediate_code::OptLevel::None)
        .or(Err("error"))
        .and(Ok(()))
}

pub fn run() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    let unit_tests = collect_tests_in_path(UNIT_TEST_PATH)?.into_iter();
    let program_tests = collect_tests_in_path(PROGRAM_TEST_PATH)?.into_iter();
    let lex_tests = collect_tests_in_path(SYNTAX_TEST_PATH)?.into_iter();

    for test in unit_tests
        .chain(program_tests)
        .chain(lex_tests)
        // Any code errors should be caught during syntax analysis,
        // we therefore only test successfully syntactically parsed inputs
        .filter(|t| t.expectation == Expectation::Success)
    {
        stats.total += 1;
        if run_single_test(test, test_function).is_ok() {
            stats.success += 1;
        }
    }
    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
