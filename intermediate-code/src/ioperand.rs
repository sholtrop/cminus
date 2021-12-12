use std::fmt;

use syntax::{ConstantNodeValue, ReturnType};

use crate::id::ISymbolId;

pub enum IOperand {
    Unknown,
    Immediate {
        value: ConstantNodeValue,
        ret_type: ReturnType,
    },
    Symbol {
        id: ISymbolId,
        ret_type: ReturnType,
    },
}

impl fmt::Display for IOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Immediate { ret_type, value } => write!(f, "imm:{} {}", ret_type, value),
            Self::Symbol { id, ret_type } => write!(f, "sym:{} {}", ret_type, id),
        }
    }
}
