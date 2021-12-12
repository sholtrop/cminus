use crate::istatement::IStatement;
use std::fmt;

pub struct IntermediateCode {
    program_name: String,
    statements: Vec<IStatement>,
}

impl IntermediateCode {
    pub fn new(program_name: String) -> IntermediateCode {
        IntermediateCode {
            program_name,
            statements: vec![],
        }
    }

    pub fn n_statements(&self) -> usize {
        self.statements.len()
    }
}

impl fmt::Display for IntermediateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program: {}", self.program_name)?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}
