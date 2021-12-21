use std::collections::HashMap;
use syntax::SymbolId;

pub struct ICInfo {
    leaders: Vec<SymbolId>,
    labels: HashMap<SymbolId, ICLineNumber>,
    calls: HashMap<SymbolId, Vec<ICLineNumber>>,
    funcs: HashMap<ICLineNumber, SymbolId>,
}

impl ICInfo {
    pub fn add_call(&mut self, id: SymbolId, line: ICLineNumber) {
        self.calls
            .entry(id)
            .and_modify(|vec| vec.push(line))
            .or_insert_with(Vec::new);
    }
}

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub struct ICLineNumber(pub usize);

impl fmt::Display for ICLineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for ICLineNumber {
    fn from(line: usize) -> Self {
        Self(line)
    }
}
