use std::fs;
use std::io;
use std::path::PathBuf;

const INCORRECT_TEST_DIR: &str = "incorrect";
const PROGRAM_DIR: &str = "programs";

#[derive(Clone, Copy)]
pub enum TestStage {
    Lexical,
    Syntax,
    Intermediate,
    Machine,
}

#[derive(PartialEq, Eq)]
pub enum Expectation {
    Success,
    Fail,
}

pub struct Test {
    pub name: String,
    pub path: PathBuf,
    pub expectation: Expectation,
}

pub struct TestStats {
    pub total: usize,
    pub success: usize,
}

pub struct TestFailed {}

impl From<()> for TestFailed {
    fn from(_: ()) -> Self {
        TestFailed {}
    }
}

pub type TestFunction = fn(&str) -> Result<(), &str>;

pub fn collect_tests_in_path(path: impl Into<PathBuf>) -> io::Result<Vec<Test>> {
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

pub fn run_single_test(test: Test, test_func: TestFunction) -> Result<(), TestFailed> {
    println!();
    log::info!("Running test {}", test.name);
    let input = std::fs::read_to_string(test.path).or(Err(TestFailed {}))?;
    let result = test_func(&input);
    match (result, test.expectation) {
        (Ok(_), Expectation::Success) | (Err(_), Expectation::Fail) => {
            log::info!("↪    PASSED");
            Ok(())
        }
        (Err(_), Expectation::Success) => {
            log::error!("↪   FAILED\nExpected Success, got Fail");
            Err(().into())
        }
        (Ok(_), Expectation::Fail) => {
            log::error!("↪   FAILED\nExpected Fail, got Success");
            Err(().into())
        }
    }
}
