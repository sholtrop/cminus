use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
};

use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    symbol::Symbol,
};

pub struct Scope {
    pub symbols: HashMap<SymbolName, SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}

pub struct ScopeManager {
    scope_stack: Vec<Scope>,
}

impl ScopeManager {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![Scope::new()],
        }
    }

    pub fn enter_new_scope(&mut self) {
        self.scope_stack.push(Scope {
            symbols: HashMap::new(),
        })
    }

    pub fn leave_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn add_symbol(&mut self, id: SymbolId, name: SymbolName) -> Result<(), SyntaxBuilderError> {
        match self
            .scope_stack
            .last_mut()
            .ok_or_else(|| SyntaxBuilderError::from("Invariant violated: Scope stack empty"))?
            .symbols
            .entry(name.clone())
        {
            Entry::Occupied(_) => Err(format!("Symbol {} redefined in current scope", name)
                .as_str()
                .into()),
            Entry::Vacant(e) => {
                e.insert(id);
                Ok(())
            }
        }
    }

    pub fn get_symbol_id(&self, name: &SymbolName) -> Option<SymbolId> {
        Some(*self.scope_stack.last()?.symbols.get(name)?)
    }

    pub fn symbol_is_defined(&self, name: &SymbolName) -> bool {
        self.get_symbol_id(name).is_some()
    }
}
