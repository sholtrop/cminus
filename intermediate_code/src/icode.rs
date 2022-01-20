use crate::{ic_info::ICLineNumber, istatement::IStatement};
use std::{collections::VecDeque, fmt};

#[derive(Default)]
pub struct IntermediateCode {
    // An index in the statements list might temporarily be None (e.g. during dead code elimination)
    // but will never be None when actually read.
    statements: VecDeque<Option<IStatement>>,
}

impl IntermediateCode {
    pub fn new() -> IntermediateCode {
        IntermediateCode {
            statements: VecDeque::new(),
        }
    }

    pub fn n_statements(&self) -> usize {
        self.statements.len()
    }

    pub fn append_statement(&mut self, statement: IStatement) {
        self.statements.push_back(Some(statement));
    }

    pub fn insert_statement(&mut self, statement: IStatement, line: ICLineNumber) {
        self.statements.insert(line.0 - 1, Some(statement));
    }

    /// Filter out all the temporarily `None` statements
    pub fn filter_none(&mut self) {
        self.statements = self
            .statements
            .iter()
            .filter(|s| s.is_some())
            .cloned()
            .collect();
    }

    pub fn remove_statement(&mut self, line: ICLineNumber) {
        *self.statements.get_mut(line.0 - 1).unwrap() = None;
    }

    /// [ICLineNumber] (not index) means '1' is the first statement
    pub fn get_statement(&self, line: ICLineNumber) -> &IStatement {
        self.statements[line.0 - 1].as_ref().unwrap()
    }

    pub fn get_statements(&self, start: ICLineNumber, end: ICLineNumber) -> Vec<&IStatement> {
        let mut vec = vec![];
        for i in self.statements.range(start.0 - 1..end.0 - 1) {
            vec.push(i.as_ref().unwrap());
        }
        vec
    }

    pub fn get_last_statement(&self) -> &IStatement {
        let last = ICLineNumber(self.n_statements());
        self.get_statement(last)
    }
}

impl fmt::Display for IntermediateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line, statement) in self.into_iter() {
            writeln!(f, "{:<3} {}", format!("{}", line), format!("{}", statement))?;
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
            let line = self.index.into();
            self.index += 1;
            Some((line, self.icode.get_statement(line)))
        }
    }
}
