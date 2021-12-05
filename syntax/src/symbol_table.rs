use itertools::Itertools;

use crate::{
    id::*,
    symbol::{Symbol, SymbolType},
};
use std::borrow::Borrow;
use std::{collections::HashMap, fmt};

#[derive(Clone, Copy)]
pub enum SymbolScope {
    Local { owning_function: SymbolId },
    Global,
}

#[derive(Clone)]
pub struct SymbolInfo {
    pub id: SymbolId,
    pub symbol_scope: SymbolScope,
    pub symbol: Symbol,
}

#[derive(Clone)]
pub struct FunctionInfo {
    pub variables: Vec<SymbolId>,
    pub parameters: Vec<SymbolId>,
}

#[derive(Default, Clone)]
pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolInfo>,
    functions: HashMap<SymbolId, FunctionInfo>,
    id_count: usize,
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#################### Functions ###################")?;
        for (id, info) in self.functions.borrow().iter() {
            let SymbolInfo { symbol, .. } = self.symbols.get(id).unwrap();
            write!(f, "{} {} (", symbol.return_type, symbol.name)?;
            if info.parameters.is_empty() {
                write!(f, "void")?;
            } else {
                let params = &info.parameters;
                let mut param_iter = params.iter();
                let first = param_iter.next().unwrap();
                let SymbolInfo { symbol, .. } = self.symbols.get(first).unwrap();
                write!(f, "{:5} {}", symbol.return_type, symbol.name)?;
                for param in param_iter {
                    let SymbolInfo { symbol, .. } = self.symbols.get(param).unwrap();
                    write!(f, ", {} {}", symbol.return_type, symbol.name)?;
                }
            }
            write!(f, ")")?;
            writeln!(f)?;
        }

        writeln!(f, "\n#################### Symbols #####################")?;
        writeln!(
            f,
            "{:<13} {:<13} {:<13} {:<13}",
            "ID", "Line", "Type", "Name"
        )?;
        for (id, info) in self
            .symbols
            .borrow()
            .iter()
            .sorted_by(|(a, _), (b, _)| a.0.cmp(&b.0))
        {
            let symbol = &info.symbol;
            if symbol.symbol_type != SymbolType::Function && symbol.line > 0 {
                writeln!(
                    f,
                    "{:<13} {:<13} {:<13} {:<13}",
                    format!("{}.", id.0),
                    format!("{}", symbol.line),
                    format!("{}", symbol.return_type),
                    format!("{}", symbol.name)
                )?;
            }
        }
        Ok(())
    }
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            functions: HashMap::new(),
            id_count: 0,
        }
    }

    pub fn get_symbol(&self, id: &SymbolId) -> Option<&Symbol> {
        Some(&self.symbols.get(id)?.symbol)
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

        // If the symbol we added is a parameter/variable belonging to a function,
        // add it to the FunctionInfo.
        if let SymbolScope::Local {
            owning_function: owning_function_id,
        } = scope
        {
            let func_info = self
                .functions
                .get_mut(&owning_function_id)
                .expect("Invariant violated: Function id not found");
            match sym_type {
                SymbolType::Parameter | SymbolType::ArrayParam => {
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

    pub fn get_func_param_ids(&self, func_id: &SymbolId) -> Option<&Vec<SymbolId>> {
        Some(&self.functions.get(func_id)?.parameters)
    }

    pub fn get_func_param_symbols(&self, func_id: &SymbolId) -> Option<Vec<Symbol>> {
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
