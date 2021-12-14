use std::fmt;

use syntax::{ConstantNodeValue, ReturnType, SymbolId};

#[derive(Clone)]
pub enum IOperand {
    Unknown,
    Immediate {
        value: ConstantNodeValue,
        ret_type: ReturnType,
    },
    Symbol {
        id: SymbolId,
        ret_type: ReturnType,
    },
}

impl IOperand {
    pub fn id(&self) -> SymbolId {
        match self {
            IOperand::Symbol { id, .. } => *id,
            _ => panic!("id called on non-symbol operand"),
        }
    }

    pub fn ret_type(&self) -> ReturnType {
        match self {
            IOperand::Immediate { ret_type, .. } | IOperand::Symbol { ret_type, .. } => *ret_type,
            _ => panic!("ret_type called on unknown operand"),
        }
    }
}

impl fmt::Display for IOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Immediate { ret_type, value } => write!(f, "imm:{} {}", ret_type, value),
            Self::Symbol { id, .. } => write!(f, "sym:{}", id),
        }
    }
}
