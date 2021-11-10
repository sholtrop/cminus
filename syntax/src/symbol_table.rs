use crate::id::*;
use std::collections::HashMap;

pub struct SymbolInfo {}

pub struct FunctionInfo {
    pub variables: Vec<VariableId>,
    pub parameters: Vec<ParameterId>,
}

pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolInfo>,
    functions: HashMap<FunctionId, FunctionInfo>,
}

impl SymbolTable {
    pub fn add_symbol() {}
}
