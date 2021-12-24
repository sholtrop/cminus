use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};
use syntax::SymbolId;
#[derive(Default, Debug)]
pub struct ICInfo {
    pub leaders: HashSet<ICLineNumber>,
    pub labels: HashMap<SymbolId, ICLineNumber>,
    pub calls: HashMap<SymbolId, Vec<ICLineNumber>>,
    pub funcs: HashMap<ICLineNumber, SymbolId>,
}

impl ICInfo {
    pub fn new() -> Self {
        Self {
            leaders: HashSet::new(),
            labels: HashMap::new(),
            calls: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
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

impl Add<usize> for ICLineNumber {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
