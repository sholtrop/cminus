use std::{
    collections::{BTreeSet, HashMap},
    fmt,
    ops::{Add, Sub},
};
use syntax::SymbolId;
#[derive(Default, Debug)]
pub struct ICInfo {
    pub leaders: BTreeSet<ICLineNumber>,
    pub labels: HashMap<SymbolId, ICLineNumber>,
    pub calls: HashMap<SymbolId, Vec<ICLineNumber>>,
    pub funcs: HashMap<SymbolId, ICLineNumber>,
    pub returns: HashMap<SymbolId, Vec<ICLineNumber>>,
}

impl ICInfo {
    pub fn add_call(&mut self, id: SymbolId, line: ICLineNumber) {
        self.calls
            .entry(id)
            .and_modify(|vec| vec.push(line))
            .or_insert_with(|| vec![line]);
    }

    pub fn add_return(&mut self, id: SymbolId, line: ICLineNumber) {
        self.returns
            .entry(id)
            .and_modify(|vec| vec.push(line))
            .or_insert_with(|| vec![line]);
    }
}

impl From<&IntermediateCode> for ICInfo {
    fn from(icode: &IntermediateCode) -> Self {
        let mut info = Self {
            ..Default::default()
        };
        let mut statements = icode.into_iter().peekable();
        let mut current_func = None;
        while let Some((line, stmt)) = statements.next() {
            if stmt.is_label() {
                info.leaders.insert(line);
                let id = stmt.label_id();
                info.labels.insert(id, line);
            } else if stmt.is_func() {
                info.leaders.insert(line);
                let id = stmt.label_id();
                info.funcs.insert(id, line);
                current_func = Some(id);
            } else if stmt.is_non_builtin_call() {
                let id = stmt.label_id();
                info.add_call(id, line);
                if statements.peek().is_some() {
                    info.leaders.insert(line + 1);
                }
            } else if stmt.is_return() {
                info.add_return(current_func.unwrap(), line);
            }
            if stmt.is_jump() && statements.peek().is_some() {
                info.leaders.insert(line + 1);
            }
        }
        info
    }
}

use crate::icode::IntermediateCode;

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug, PartialOrd, Ord)]
pub struct ICLineNumber(pub usize);

impl fmt::Display for ICLineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for ICLineNumber {
    fn from(line: usize) -> Self {
        Self(line + 1)
    }
}

impl From<ICLineNumber> for usize {
    fn from(line: ICLineNumber) -> Self {
        line.0 - 1
    }
}

impl Add<usize> for ICLineNumber {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for ICLineNumber {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}
