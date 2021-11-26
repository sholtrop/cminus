use std::io;
use tests::{collect_tests_in_path, run_single_test, TestStats};

const PROGRAM_TEST_PATH: &str = "tests/testfiles/general/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/general/units";
const SYNTAX_TEST_PATH: &str = "tests/testfiles/syntax";

pub fn test_function(input: &str) -> Result<(), &str> {
    let parsed = lexical::parse(input).unwrap_or_else(|_| {
        panic!("Could not lexically parse file for syntax test");
    });
    syntax::generate(parsed).and(Ok(())).or(Err("Failed"))
}

pub fn run() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    let unit_tests = collect_tests_in_path(UNIT_TEST_PATH)?.into_iter();
    let program_tests = collect_tests_in_path(PROGRAM_TEST_PATH)?.into_iter();
    let lex_tests = collect_tests_in_path(SYNTAX_TEST_PATH)?.into_iter();

    for test in unit_tests.chain(program_tests).chain(lex_tests) {
        stats.total += 1;
        if run_single_test(test, test_function).is_ok() {
            stats.success += 1;
        }
    }

    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
