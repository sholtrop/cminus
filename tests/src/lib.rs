use std::path::PathBuf;

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
