use std::collections::HashMap;

use crate::id::{SymbolId, SymbolName};

pub struct Scope {
    symbols: HashMap<SymbolName, SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}
