use std::collections::hash_map::Entry;

use lexical::{ParseTree, Rule};

use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    node::{NodeType, SyntaxNode},
    scope::{Scope, ScopeManager},
    symbol::{ReturnType, Symbol, SymbolType},
    symbol_table::{SymbolScope, SymbolTable},
    syntax_tree::{FunctionRoot, SyntaxTree},
    visitor::{SyntaxResult, Visitor},
};

pub struct SyntaxBuilder {
    table: SymbolTable,
    tree: SyntaxTree,
    scope_manager: ScopeManager,
    current_function: Option<SymbolId>,
    current_line: usize,
}

impl SyntaxBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            tree: SyntaxTree::new(),
            scope_manager: ScopeManager::new(),
            current_function: None,
            current_line: 1,
        }
    }

    /// Return the produced [SyntaxResult]. This consumes the `self` value of the [SyntaxBuilder].
    pub fn result(self) -> SyntaxResult {
        SyntaxResult {
            symbol_table: self.table,
            tree: self.tree,
        }
    }

    pub fn add_builtins(&mut self) {
        todo!("implement")
    }

    fn add_function(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        if self.scope_manager.symbol_is_defined(&symbol.name) {
            return Err(
                format!("Function {} redefined in current scope", symbol.name)
                    .as_str()
                    .into(),
            );
        }
        let id = self.table.add_function(symbol.clone());
        self.scope_manager.add_symbol(id, symbol.name)?;
        Ok(id)
    }

    /// Create and enter a new function.
    pub fn enter_function(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        let id = self.add_function(Symbol {
            name: name.clone(),
            return_type: return_type.clone(),
            symbol_type: SymbolType::Function,
            line: self.current_line,
        })?;
        self.current_function = Some(id);
        self.tree
            .functions
            .insert(id, FunctionRoot { name, tree: None });
        Ok(id)
    }

    pub fn leave_function(&mut self) {
        self.current_function = None;
    }

    pub fn get_symbol_by_id(&self, id: SymbolId) -> Option<&Symbol> {
        self.table.get_symbol(&id)
    }

    pub fn get_symbol_by_name(&self, name: &SymbolName) -> Option<&Symbol> {
        let id = self.get_symbol_id(name)?;
        let symbol = self.get_symbol_by_id(id).unwrap_or_else(|| {
            panic!(
                "SymbolId {:?} was found by the scope manager but was not in the symbol table",
                id
            )
        });
        Some(symbol)
    }

    pub fn get_symbol_id(&self, name: &SymbolName) -> Option<SymbolId> {
        self.scope_manager.get_symbol_id(name)
    }

    pub fn get_current_function(&self) -> Option<SymbolId> {
        self.current_function
    }

    pub fn attach_root(
        &mut self,
        func_id: &SymbolId,
        new_root: SyntaxNode,
    ) -> Result<(), SyntaxBuilderError> {
        let func_root = self.tree.functions.get_mut(func_id).ok_or_else(|| {
            SyntaxBuilderError(format!(
                "Cannot attach root: Function with id {:?} not found",
                func_id
            ))
        })?;
        func_root.tree = Some(new_root);
        todo!("Implement")
    }

    pub fn get_parameters(&self, name: &SymbolName) -> Option<Vec<Symbol>> {
        let id = self.get_symbol_id(name)?;
        self.table.get_func_param_symbols(id)
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        if self.scope_manager.symbol_is_defined(&symbol.name) {
            return Err(format!("Symbol {} redefined in current scope", symbol.name)
                .as_str()
                .into());
        }
        let scope = if let Some(id) = self.current_function {
            SymbolScope::Local {
                owning_function: id,
            }
        } else {
            SymbolScope::Global
        };
        Ok(self.table.add_symbol(symbol, scope))
    }

    pub fn enter_new_scope(&mut self) {
        self.scope_manager.enter_new_scope()
    }

    pub fn leave_scope(&mut self) {
        self.scope_manager.leave_scope()
    }
}
