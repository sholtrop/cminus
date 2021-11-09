use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::PathBuf;

const CORRECT_TEST_DIR: &str = "correct";
const PROGRAM_DIR: &str = "programs";
const PROGRAM_TEST_PATH: &str = "tests/testfiles/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/units";

#[derive(Clone, Copy)]
pub enum TestStage {
    Lexical,
    Syntax,
    Intermediate,
    Machine,
}

impl From<TestStage> for TestFunction {
    fn from(stage: TestStage) -> Self {
        match stage {
            TestStage::Lexical => |input| {
                lexical::parse(&input)?;
                Ok(())
            },
            _ => todo!("implement"),
        }
    }
}

impl TestStage {
    pub fn get_paths(&self) -> Vec<PathBuf> {
        let mut paths = vec![
            PathBuf::from(PROGRAM_TEST_PATH),
            PathBuf::from(UNIT_TEST_PATH),
        ];
        match self {
            TestStage::Lexical => {
                paths.push(PathBuf::from("lexical/tests"));
            }
            _ => {
                todo!("implement")
            }
        }
        paths
    }
}

#[derive(PartialEq, Eq)]
pub enum Expectation {
    Success,
    Fail,
}

type TestFunction = fn(String) -> Result<(), Box<dyn std::error::Error>>;

pub struct Test {
    name: String,
    path: PathBuf,
    expectation: Expectation,
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

pub fn run_test(test: Test, test_function: TestFunction) -> Result<(), TestFailed> {
    let file_contents = fs::read_to_string(&test.path).unwrap();
    let result = test_function(file_contents);
    match (result, test.expectation) {
        (Ok(()), Expectation::Success) => {
            log::info!("PASSED {}", test.name);
            Ok(())
        }
        _ => {
            log::error!("FAILED {}", test.name);
            Err(().into())
        }
    }
}

pub fn collect_tests_in_path(
    path: impl Into<PathBuf>,
    stats: &mut TestStats,
) -> io::Result<Vec<Test>> {
    let mut tests: Vec<Test> = Vec::new();
    for entry in fs::read_dir(path.into())? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            tests.append(&mut collect_tests_in_path(entry.path(), stats)?);
        } else {
            let file = entry.file_name().into_string().unwrap();
            if file.ends_with(".c") {
                let expectation = if entry.path().ancestors().any(|path| {
                    let p = path.to_str().unwrap();
                    p.ends_with(CORRECT_TEST_DIR) || p.ends_with(PROGRAM_DIR)
                }) {
                    Expectation::Success
                } else {
                    Expectation::Fail
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
