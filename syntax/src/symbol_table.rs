use crate::{
    id::*,
    symbol::{Symbol, SymbolType},
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum SymbolScope {
    Local { owning_function_id: SymbolId },
    Global,
}

pub struct SymbolInfo {
    pub id: SymbolId,
    pub symbol_scope: SymbolScope,
    pub symbol: Symbol,
}

pub struct FunctionInfo {
    pub variables: Vec<SymbolId>,
    pub parameters: Vec<SymbolId>,
}

pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolInfo>,
    functions: HashMap<SymbolId, FunctionInfo>,
    id_count: usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            functions: HashMap::new(),
            id_count: 0,
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol, scope: SymbolScope) -> SymbolId {
        let id = SymbolId(self.id_count);
        let sym_type = symbol.symbol_type;
        self.symbols.insert(
            id,
            SymbolInfo {
                id,
                symbol,
                symbol_scope: scope,
            },
        );

        if let SymbolScope::Local { owning_function_id } = scope {
            let func_info = self
                .functions
                .get_mut(&owning_function_id)
                .expect("Invariant violated: Function id not found");
            match sym_type {
                SymbolType::Parameter => {
                    func_info.parameters.push(id);
                }
                SymbolType::Variable => {
                    func_info.variables.push(id);
                }
                _ => {}
            };
        }

        self.id_count += 1;
        id
    }

    pub fn add_function(&mut self, function: Symbol) -> SymbolId {
        let id = self.add_symbol(function, SymbolScope::Global);
        self.functions.insert(
            id,
            FunctionInfo {
                parameters: vec![],
                variables: vec![],
            },
        );

        id
    }

    pub fn get_func_param_ids(&self, func_id: SymbolId) -> Option<&Vec<SymbolId>> {
        Some(&self.functions.get(&func_id)?.parameters)
    }

    pub fn get_func_param_symbols(&self, func_id: SymbolId) -> Option<Vec<Symbol>> {
        let mut symbols = Vec::new();
        for id in self.get_func_param_ids(func_id)? {
            symbols.push(self.symbols.get(id)?.symbol.clone());
        }
        Some(symbols)
    }

    pub fn get_func_var_ids(&self, func_id: SymbolId) -> Option<&Vec<SymbolId>> {
        let vars = &self.functions.get(&func_id)?.variables;
        Some(vars)
    }

    pub fn get_func_var_symbols(&self, func_id: SymbolId) -> Option<Vec<Symbol>> {
        let mut symbols = Vec::new();
        for id in self.get_func_var_ids(func_id)? {
            symbols.push(self.symbols.get(id)?.symbol.clone());
        }
        Some(symbols)
    }
}
