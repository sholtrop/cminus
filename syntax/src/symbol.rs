use crate::id::SymbolName;
use core::fmt;
use std::convert::From;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Copy)]
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
    Label,
}

impl From<&ReturnType> for usize {
    fn from(ret: &ReturnType) -> Self {
        match ret {
            ReturnType::Error => 0,
            ReturnType::Bool => 1,
            ReturnType::Int8 => 2,
            ReturnType::Uint8 => 3,
            ReturnType::Int => 4,
            ReturnType::Uint => 5,
            _ => 6, // Other types cannot be coerced
        }
    }
}

impl PartialOrd for ReturnType {
    /// Defines a partial ordering for the purpose of type coercion as follows:
    /// ```
    /// From      To
    /// UINT      BOOL
    /// INT       UINT, BOOL
    /// UINT8     INT, UINT, BOOL
    /// INT8      UINT8, INT, UINT, BOOL
    /// BOOL      INT8, UINT8, INT, UINT
    /// ```
    /// If `a < b` then `a` can be coerced to `b`
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order_left: usize = self.into();
        let order_right: usize = other.into();
        order_left.partial_cmp(&order_right)
    }
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
                ReturnType::Label => "label",
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
    pub fn to_array_type(self) -> Self {
        match self {
            Self::Int => Self::IntArray,
            Self::Int8 => Self::Int8Array,
            Self::Uint => Self::UintArray,
            Self::Uint8 => Self::Uint8Array,
            _ => Self::Error,
        }
    }

    pub fn to_base_type(self) -> Self {
        match self {
            ReturnType::Int8Array => ReturnType::Int8,
            ReturnType::Uint8Array => ReturnType::Uint8,
            ReturnType::IntArray => ReturnType::Int,
            ReturnType::UintArray => ReturnType::Uint,
            _ => self,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(self, Self::Uint | Self::Uint8)
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
    ArrayVariable { size: usize },
    ArrayParam,
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolType::Unknown => "unknown".into(),
                SymbolType::Error => "error".into(),
                SymbolType::Variable => "variable".into(),
                SymbolType::Parameter => "parameter".into(),
                SymbolType::Function => "function".into(),
                SymbolType::Program => "program".into(),
                SymbolType::TempVar => "tempvar".into(),
                SymbolType::Label => "label".into(),
                SymbolType::ArrayVariable { size } => format!("arrayvar({})", size),
                SymbolType::ArrayParam => "arrayparam".into(),
            }
        )
    }
}
#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: SymbolName,
    pub return_type: ReturnType,
    pub symbol_type: SymbolType,
    pub line: usize,
}

impl Symbol {
    pub fn is_array(&self) -> bool {
        matches!(
            self.return_type,
            ReturnType::Int8Array
                | ReturnType::IntArray
                | ReturnType::Uint8Array
                | ReturnType::UintArray
        )
    }
    pub fn is_param(&self) -> bool {
        matches!(
            self.symbol_type,
            SymbolType::ArrayParam | SymbolType::Parameter
        )
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[`{}` Ret:{} Type:{}]",
            self.name.0, self.return_type, self.symbol_type
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
