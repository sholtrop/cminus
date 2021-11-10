use core::fmt;

#[derive(PartialEq)]
pub enum ReturnType {
    Unknown,
    Error,
    Void,
    Int,
    IntArray,
    Int8,
    Int8Array,
    Uint,
    UintArray,
    Uint8,
    Uint8Array,
    Real,
    Bool,
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReturnType::Unknown => "unknown",
                ReturnType::Error => "error",
                ReturnType::Void => "void",
                ReturnType::Int => "int",
                ReturnType::IntArray => "int_array",
                ReturnType::Int8 => "int8",
                ReturnType::Int8Array => "int8_array",
                ReturnType::Uint => "uint",
                ReturnType::UintArray => "uint_array",
                ReturnType::Uint8 => "uint8",
                ReturnType::Uint8Array => "uint8_array",
                ReturnType::Real => "real",
                ReturnType::Bool => "bool",
            }
        )
    }
}

#[derive(PartialEq)]
pub enum SymbolType {
    Unknown,
    Error,
    Variable,
    Parameter,
    Function,
    Program,
    TempVar,
    Label,
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolType::Unknown => "unknown",
                SymbolType::Error => "error",
                SymbolType::Variable => "variable",
                SymbolType::Parameter => "parameter",
                SymbolType::Function => "function",
                SymbolType::Program => "program",
                SymbolType::TempVar => "tempvar",
                SymbolType::Label => "label",
            }
        )
    }
}

pub struct Symbol {
    name: String,
    return_type: ReturnType,
    symbol_type: SymbolType,
    line: usize,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {} {}]",
            self.name, self.return_type, self.symbol_type
        )
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        let Symbol {
            line,
            name,
            return_type,
            symbol_type,
        } = other;
        self.line == *line
            && self.name == *name
            && self.return_type == *return_type
            && self.symbol_type == *symbol_type
    }
}
