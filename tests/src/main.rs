use std::fs;
use std::io;
use std::path::{Ancestors, Path, PathBuf};

use general::logging::init_logger;

const PROGRAM_TEST_PATH: &str = "tests/testfiles/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/units";
const CORRECT_TEST_DIR: &str = "correct";

#[derive(PartialEq, Eq)]
enum Expectation {
    Success,
    Fail,
}

type TestFunction = fn(String) -> Result<(), ()>;

struct Test {
    name: String,
    file_contents: String,
    expectation: Expectation,
    test_fn: TestFunction,
}

struct TestStats {
    total: usize,
    success: usize,
}

fn run_test(test: Test) -> Result<(), ()> {
    let result = (test.test_fn)(test.file_contents);
    match (result, test.expectation) {
        (Ok(()), Expectation::Success) => {
            log::info!("PASSED {}", test.name);
            Ok(())
        }
        _ => {
            log::error!("FAILED {}", test.name);
            Err(())
        }
    }
}

fn program_tests() -> io::Result<()> {
    let mut ancestors = Path::new("/foo/bar").ancestors();
    // let x = ancestors.  //.try_find(|p| p == Path::new("/foo"))

    for entry in fs::read_dir(PROGRAM_TEST_PATH).unwrap() {
        let entry = entry?;
        todo!("implement");
    }
    Ok(())
}

fn unit_tests(path: impl Into<PathBuf>, stats: &mut TestStats) -> io::Result<()> {
    for entry in fs::read_dir(path.into())? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            unit_tests(entry.path(), stats)?;
        } else {
            let file = entry.file_name().into_string().unwrap();
            if file.ends_with(".c") {
                let expectation = if entry
                    .path()
                    .ancestors()
                    .any(|path| path.to_str().unwrap().ends_with(CORRECT_TEST_DIR))
                {
                    Expectation::Success
                } else {
                    Expectation::Fail
                };

                if let Ok(()) = run_test(Test {
                    expectation,
                    file_contents: fs::read_to_string(entry.path())?,
                    name: entry.path().into_os_string().into_string().unwrap(),
                    test_fn: |c| Ok(()),
                }) {
                    stats.success += 1;
                }
                stats.total += 1;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    init_logger();
    unit_tests(UNIT_TEST_PATH, &mut stats)?;

    log::info!("[{} / {}] TESTS PASSED", stats.success, stats.total);

    // program_tests()?;
    Ok(())
}
