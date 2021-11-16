use std::{fs, io, path::PathBuf};

use crate::lib::{Expectation, Test, TestFailed, TestStats};

const INCORRECT_TEST_DIR: &str = "incorrect";
const PROGRAM_DIR: &str = "programs";
const PROGRAM_TEST_PATH: &str = "tests/testfiles/general/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/general/units";
const LEXICAL_TEST_PATH: &str = "tests/testfiles/lexical";

fn collect_tests_in_path(path: impl Into<PathBuf>) -> io::Result<Vec<Test>> {
    let mut tests: Vec<Test> = Vec::new();
    for entry in fs::read_dir(path.into())? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            tests.append(&mut collect_tests_in_path(entry.path())?);
        } else {
            let file = entry.file_name().into_string().unwrap();
            if file.ends_with(".c") {
                let expectation = if entry.path().ancestors().any(|path| {
                    let p = path.to_str().unwrap();
                    p.ends_with(INCORRECT_TEST_DIR) && !p.ends_with(PROGRAM_DIR)
                }) {
                    Expectation::Fail
                } else {
                    Expectation::Success
                };
                tests.push(Test {
                    expectation,
                    name: entry.path().into_os_string().into_string().unwrap(),
                    path: entry.path(),
                });
            }
        }
    }
    Ok(tests)
}

fn run_single_test(test: Test) -> Result<(), TestFailed> {
    let input = std::fs::read_to_string(test.path).or(Err(TestFailed {}))?;
    let result = lexical::parse(&input);
    match (result, test.expectation) {
        (Ok(_), Expectation::Success) => {
            log::info!("PASSED {}", test.name);
            Ok(())
        }
        (Err(_), Expectation::Fail) => {
            log::info!("PASSED {}", test.name);
            Ok(())
        }
        (Err(_), Expectation::Success) => {
            log::error!("FAILED {}\nExpected Success, got Fail", test.name);
            Err(().into())
        }
        (Ok(_), Expectation::Fail) => {
            log::error!("FAILED {}\nExpected Fail, got Success", test.name);
            Err(().into())
        }
    }
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
        if run_single_test(test).is_ok() {
            stats.success += 1;
        }
    }

    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);
    Ok(())
}
