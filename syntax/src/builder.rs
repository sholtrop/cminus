use std::borrow::Borrow;
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
    visitor::{SyntaxAnalysisResult, Visitor},
};

pub struct SyntaxBuilder {
    table: SymbolTable,
    tree: SyntaxTree,
    scope_manager: ScopeManager,
    current_function: Option<SymbolId>,
}

impl SyntaxBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            tree: SyntaxTree::new(),
            scope_manager: ScopeManager::new(),
            current_function: None,
        }
    }

    /// Return the produced [SyntaxResult]. This consumes the `self` value of the [SyntaxBuilder].
    pub fn result(self) -> (SymbolTable, SyntaxTree) {
        (self.table, self.tree)
    }

    /// Register a new function and return its [SymbolId].
    /// Returns an error if a function with the given name is already defined.
    fn add_function(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        if self.scope_manager.symbol_is_defined(&symbol.name) {
            return Err(format!("Function with name {} is already defined", symbol.name).into());
        }
        let id = self.table.add_function(symbol.clone());
        self.scope_manager.add_symbol(id, symbol.name)?;
        Ok(id)
    }

    /// Create and enter a new function.
    pub fn enter_function(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        let name = symbol.name.clone();
        let id = self.add_function(symbol)?;
        self.current_function = Some(id);
        self.tree
            .functions
            .insert(id, FunctionRoot { name, tree: None });
        Ok(id)
    }

    pub fn leave_function(&mut self) {
        self.current_function = None;
    }

    pub fn get_symbol_by_id(&self, id: &SymbolId) -> Option<&Symbol> {
        self.table.get_symbol(id)
    }

    pub fn get_symbol_by_name(&self, name: &SymbolName) -> Option<(&Symbol, SymbolId)> {
        let id = self.get_symbol_id(name)?;
        let symbol = self.get_symbol_by_id(&id).unwrap_or_else(|| {
            panic!(
                "SymbolId {:?} was found by the scope manager but was not in the symbol table",
                id
            )
        });
        Some((symbol, id))
    }

    pub fn get_symbol_id(&self, name: &SymbolName) -> Option<SymbolId> {
        self.scope_manager.get_symbol_id(name)
    }

    pub fn get_current_function(&self) -> Option<&Symbol> {
        if let Some(id) = self.current_function {
            self.get_symbol_by_id(&id)
        } else {
            None
        }
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
        Ok(())
    }

    pub fn get_parameters(&self, id: &SymbolId) -> Result<Vec<Symbol>, SyntaxBuilderError> {
        self.table.get_func_param_symbols(id).ok_or_else(|| {
            SyntaxBuilderError(format!("Symbol with id {} not found in function table", id))
        })
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        if self.scope_manager.symbol_is_defined(&symbol.name) {
            return Err(format!("Symbol `{}` redefined in current scope", symbol.name).into());
        }

        let scope = if let Some(id) = self.current_function {
            SymbolScope::Local {
                owning_function: id,
            }
        } else {
            SymbolScope::Global
        };
        let name = symbol.borrow().name.clone();
        let id = self.table.add_symbol(symbol, scope);
        // We checked whether the symbol is defined already at the beginning of the function
        self.scope_manager.add_symbol(id, name).unwrap();
        Ok(id)
    }

    pub fn enter_new_scope(&mut self) {
        self.scope_manager.enter_new_scope()
    }

    pub fn leave_scope(&mut self) {
        self.scope_manager.leave_scope()
    }

    // Adds the parameters of the current function to the current scope.
    // Panics if there is no current function, i.e. we are in the global scope.
    // pub fn add_params_to_scope(&mut self) -> Result<(), SyntaxBuilderError> {
    //     let id = self.current_function.unwrap();
    //     for param in self.get_parameters(&id).unwrap() {
    //         self.scope_manager
    //             .add_symbol(id, param.name.clone())
    //             .map_err(|_| {
    //                 SyntaxBuilderError(format!(
    //                     "Multiple parameters with name `{}` for function {}",
    //                     param.name,
    //                     self.get_symbol_by_id(&id).unwrap().name
    //                 ))
    //             })?;
    //     }
    //     Ok(())
    // }
}
