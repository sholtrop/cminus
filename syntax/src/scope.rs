use std::collections::{hash_map::Entry, HashMap};

use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
};

#[derive(Default)]
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
#[derive(Default)]
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
        self.scope_stack.push(Scope::new());
        log::trace!("ENTER NEW SCOPE {}", self.scope_stack.len());
    }

    pub fn leave_scope(&mut self) {
        self.scope_stack.pop();
        log::trace!("LEAVE SCOPE {}", self.scope_stack.len());
    }

    pub fn add_symbol(&mut self, id: SymbolId, name: SymbolName) -> Result<(), SyntaxBuilderError> {
        match self
            .scope_stack
            .last_mut()
            .ok_or_else(|| SyntaxBuilderError::from("Invariant violated: Scope stack empty"))?
            .symbols
            .entry(name.clone())
        {
            Entry::Occupied(_) => {
                Err(format!("Symbol `{}` redefined in current scope", name).into())
            }
            Entry::Vacant(e) => {
                e.insert(id);
                Ok(())
            }
        }
    }

    /// Returns the [SymbolId] of the symbol with the given name, if it exists.
    /// Will first try the most local scope, then the scope above that etc. until the global scope.
    pub fn get_symbol_id(&self, name: &SymbolName) -> Option<SymbolId> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(id) = scope.symbols.get(name) {
                return Some(*id);
            }
        }
        log::trace!("Could not find {} in scope", name);
        None
    }

    /// Whether the symbol with `name` is defined in the current scope.
    /// So not whether `name` is also declared in higher scopes.
    pub fn symbol_is_defined(&self, name: &SymbolName) -> bool {
        self.scope_stack
            .last()
            .expect("Invariant violated: Scope stack was empty")
            .symbols
            .get(name)
            .is_some()
    }
}
