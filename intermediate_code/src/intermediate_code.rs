use crate::{ic_info::ICLineNumber, istatement::IStatement};
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

    /// Can give a negative index, which will index starting from the end of the statement list counting backwards.
    pub fn get_statement(&self, mut idx: i32) -> &IStatement {
        if idx < 0 {
            idx += self.n_statements() as i32;
        }
        &self.statements[idx as usize]
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

impl<'a> IntoIterator for &'a IntermediateCode {
    type Item = <ICodeIterator<'a> as Iterator>::Item;
    type IntoIter = ICodeIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ICodeIterator {
            icode: self,
            index: 0,
        }
    }
}

pub struct ICodeIterator<'a> {
    icode: &'a IntermediateCode,
    index: usize,
}

impl<'a> Iterator for ICodeIterator<'a> {
    type Item = (ICLineNumber, &'a IStatement);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.icode.n_statements() {
            None
        } else {
            self.index += 1;
            Some((
                ICLineNumber(self.index),
                self.icode.get_statement((self.index - 1) as i32),
            ))
        }
    }
}
