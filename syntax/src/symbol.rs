use core::fmt;

use crate::id::SymbolName;

#[derive(PartialEq, Clone, Debug)]
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

impl From<&str> for ReturnType {
    fn from(string: &str) -> Self {
        match string {
            "int" => Self::Int,
            "int8_t" => Self::Int8,
            "void" => Self::Void,
            "unsigned int" | "unsigned" => Self::Uint,
            "uint8_t" => Self::Uint8,
            _ => Self::Error,
        }
    }
}

impl ReturnType {
    fn to_array_type(&self) -> Self {
        match *self {
            Self::Int => Self::IntArray,
            Self::Int8 => Self::Int8Array,
            Self::Uint => Self::UintArray,
            Self::Uint8 => Self::Uint8Array,
            _ => Self::Error,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
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
#[derive(Clone)]
pub struct Symbol {
    pub name: SymbolName,
    pub return_type: ReturnType,
    pub symbol_type: SymbolType,
    pub line: usize,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. [{:?} Ret:{} Type:{}]",
            self.line, self.name, self.return_type, self.symbol_type
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
