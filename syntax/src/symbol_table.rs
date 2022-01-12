use itertools::Itertools;
use regex::Regex;

use crate::{
    id::*,
    symbol::{ReturnType, Symbol, SymbolType},
};
use std::borrow::Borrow;
use std::{collections::HashMap, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub const SYMBOL_ID_ERROR: usize = 0;
pub const MAIN_FN: &str = "main";
#[derive(Default, Clone)]
pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolInfo>,
    functions: HashMap<SymbolId, FunctionInfo>,
    id_count: usize,
    main: Option<SymbolId>,
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
        writeln!(f, "(Excluding builtins)")?;
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
            if !id.is_builtin() {
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
            id_count: SYMBOL_ID_ERROR + 1,
            main: None,
        }
    }

    pub fn get_symbol(&self, id: &SymbolId) -> Option<&Symbol> {
        Some(&self.symbols.get(id)?.symbol)
    }

    pub fn get_function_ids(&self) -> Vec<SymbolId> {
        self.functions
            .keys()
            .cloned()
            .sorted_by_key(|x| x.0)
            .collect()
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
        let is_main = function.name.0 == MAIN_FN;
        let id = self.add_symbol(function, SymbolScope::Global);
        self.functions.insert(
            id,
            FunctionInfo {
                parameters: vec![],
                variables: vec![],
            },
        );
        if is_main {
            self.main = Some(id);
        }
        id
    }

    pub fn get_func_param_ids(&self, func_id: &SymbolId) -> Option<&Vec<SymbolId>> {
        Some(&self.functions.get(func_id)?.parameters)
    }

    pub fn get_func_param_symbols(&self, func_id: &SymbolId) -> Vec<(&SymbolId, Symbol)> {
        let mut symbols = vec![];
        for id in self.get_func_param_ids(func_id).unwrap() {
            let sym = self.symbols.get(id).unwrap().symbol.clone();
            symbols.push((id, sym));
        }
        symbols
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

    pub fn add_label(&mut self, name: String, func_id: SymbolId) -> SymbolId {
        self.add_symbol(
            Symbol {
                name: SymbolName(name),
                symbol_type: SymbolType::Label,
                line: 0,
                return_type: ReturnType::Label,
            },
            SymbolScope::Local {
                owning_function: func_id,
            },
        )
    }

    pub fn add_tempvar(
        &mut self,
        return_type: ReturnType,
        name: String,
        func_id: SymbolId,
    ) -> SymbolId {
        self.add_symbol(
            Symbol {
                name: SymbolName(name),
                symbol_type: SymbolType::TempVar,
                line: 0,
                return_type,
            },
            SymbolScope::Local {
                owning_function: func_id,
            },
        )
    }

    /// Return all the global symbols in the [SymbolTable].
    pub fn get_globals(&self) -> HashMap<SymbolId, Symbol> {
        let mut hm = HashMap::new();
        for (id, info) in &self.symbols {
            if info.symbol_scope == SymbolScope::Global {
                hm.insert(*id, info.symbol.clone());
            }
        }
        hm
    }

    pub fn get_main_id(&self) -> SymbolId {
        self.main.expect("Error: `main` function is undefined")
    }

    pub fn annotate_icode(&self, icode: String) -> String {
        let mut annotated = String::new();
        for line in icode.split('\n') {
            for part in line.split(' ') {
                let re = Regex::new(r"\[sym:(\d+)\]").unwrap();
                if let Some(c) = re.captures(part) {
                    let num: usize = c.get(1).unwrap().as_str().parse().unwrap();
                    let symbol = self.get_symbol(&SymbolId(num)).unwrap();
                    if symbol.symbol_type == SymbolType::TempVar {
                        annotated += format!("[t{}]", symbol.name).as_str();
                    } else {
                        annotated += format!("[{}]", symbol.name).as_str();
                    }
                } else {
                    annotated += part;
                }
                annotated += " ";
            }
            annotated += "\n";
        }
        annotated
    }

    /// For tests
    pub fn has_function(&self, func_name: &str) -> bool {
        self.symbols
            .values()
            .any(|s| s.symbol.name == SymbolName::from(func_name))
    }

    /// For tests
    pub fn has_parameter(&self, func_name: &str, param_name: &str) -> bool {
        let func = match self.symbols.values().find(|s| s.symbol.name.0 == func_name) {
            Some(f) => f,
            None => return false,
        };
        self.functions
            .get(&func.id)
            .expect("No parameters for found function id")
            .parameters
            .iter()
            .any(|pid| self.get_symbol(pid).unwrap().name.0 == param_name)
    }

    /// For tests
    pub fn has_local(&self, func_name: &str, local: &str) -> bool {
        let func = match self.symbols.values().find(|s| s.symbol.name.0 == func_name) {
            Some(f) => f,
            None => return false,
        };
        self.functions
            .get(&func.id)
            .expect("No parameters for found function id")
            .variables
            .iter()
            .any(|vid| self.get_symbol(vid).unwrap().name.0 == local)
    }
}
