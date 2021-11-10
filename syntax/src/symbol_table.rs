use crate::{id::*, symbol::Symbol};
use std::collections::HashMap;

pub enum SymbolScope {
    Local { owning_function_id: FunctionId },
    Global,
}

pub struct SymbolInfo {
    pub id: SymbolId,
    pub symbol_scope: SymbolScope,
    pub symbol: Option<Symbol>,
}

pub struct FunctionInfo {
    pub variables: Vec<VariableId>,
    pub parameters: Vec<ParameterId>,
}

pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolInfo>,
    functions: HashMap<FunctionId, FunctionInfo>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_symbol() {
        todo!("Implement")
    }

    pub fn get_func_parameters_by_id(&self, func_id: FunctionId) -> Option<&Vec<ParameterId>> {
        Some(&self.functions.get(&func_id)?.parameters)
    }

    pub fn get_func_parameter_symbols(&self, func_id: FunctionId) -> Option<Vec<Symbol>> {
        let mut symbols = Vec::new();
        for id in self.get_func_parameters_by_id(func_id)?.into_iter() {
            let symbol = self.symbols.get(&(*id).into())?.symbol.clone()?;
            symbols.push(symbol);
        }
        Some(symbols)
    }
}
