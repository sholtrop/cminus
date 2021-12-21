use crate::id::ICLineNumber;
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
