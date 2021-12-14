use crate::istatement::IStatement;
use std::fmt;

#[derive(Default)]
pub struct IntermediateCode {
    statements: Vec<IStatement>,
}

impl IntermediateCode {
    pub fn new() -> IntermediateCode {
        IntermediateCode { statements: vec![] }
    }

    pub fn n_statements(&self) -> usize {
        self.statements.len()
    }

    pub fn append_statement(&mut self, statement: IStatement) {
        self.statements.push(statement);
    }

    pub fn insert_statement(&mut self, statement: IStatement, index: usize) {
        self.statements.insert(index, statement);
    }

    pub fn remove_statement(&mut self, index: usize) {
        self.statements.remove(index);
    }

    pub fn get_statement(&self, idx: usize) -> &IStatement {
        &self.statements[idx]
    }
}

impl fmt::Display for IntermediateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}
