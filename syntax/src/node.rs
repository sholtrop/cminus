use crate::symbol::ReturnType;
use core::fmt;
use ptree::TreeItem;
use std::ops::Deref;
use std::{borrow::Cow, vec};

#[derive(PartialEq, Clone)]
pub enum NodeType {
    Unknown,
    Error,
    StatementList,
    Assignment,
    If,
    IfTargets,
    While,
    LArray,
    RArray,
    Return,
    FunctionCall,
    ExpressionList,

    // Relational operators
    RelEqual,
    RelLT,
    RelGT,
    RelLTE,
    RelGTE,
    RelNotEqual,

    // Binary arithmetic & logic operators
    Add,
    Sub,
    Or,
    Mul,
    Div,
    IDiv,
    Mod,
    And,

    // Leafs
    Num,
    Id,
    Empty,

    // Unary
    Not,
    SignPlus,
    SignMinus,
    Coercion, // Int to Real coercion
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeType::Unknown => "unknown",
                NodeType::Error => "error",
                NodeType::StatementList => "statement_list",
                NodeType::Assignment => "assignment",
                NodeType::If => "if",
                NodeType::IfTargets => "if_targets",
                NodeType::While => "while",
                NodeType::LArray => "l_array",
                NodeType::RArray => "r_array",
                NodeType::Return => "return",
                NodeType::FunctionCall => "function_call",
                NodeType::ExpressionList => "expression_list",
                NodeType::RelEqual => "rel_equal",
                NodeType::RelLT => "rel_lt",
                NodeType::RelGT => "rel_gt",
                NodeType::RelLTE => "rel_lte",
                NodeType::RelGTE => "rel_gte",
                NodeType::RelNotEqual => "rel_not_equal",
                NodeType::Add => "add",
                NodeType::Sub => "sub",
                NodeType::Or => "or",
                NodeType::Mul => "mul",
                NodeType::Div => "div",
                NodeType::IDiv => "idiv",
                NodeType::Mod => "mod",
                NodeType::And => "and",
                NodeType::Num => "num",
                NodeType::Id => "id",
                NodeType::Empty => "empty",
                NodeType::Not => "not",
                NodeType::SignPlus => "sign_plus",
                NodeType::SignMinus => "sign_minus",
                NodeType::Coercion => "coercion",
            }
        )
    }
}

#[derive(PartialEq, Clone)]
pub enum ConstantNodeValue {
    Uint8(u8),
    Int8(i8),
    Int(i32),
    Unsigned(u32),
}

impl fmt::Display for ConstantNodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Uint8(v) => v.to_string(),
                Self::Int8(v) => v.to_string(),
                Self::Int(v) => v.to_string(),
                Self::Unsigned(v) => v.to_string(),
            }
        )
    }
}

#[derive(PartialEq, Clone)]
pub enum Node {
    Unary {
        node_type: NodeType,
        return_type: ReturnType,
        child: Option<Box<Node>>,
    },
    Binary {
        node_type: NodeType,
        return_type: ReturnType,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Constant {
        node_type: NodeType,
        return_type: ReturnType,
        value: ConstantNodeValue,
    },
    Symbol {
        node_type: NodeType,
        return_type: ReturnType,
        symbol_id: u32,
    },
    Empty,
}

impl TreeItem for Node {
    type Child = Self;
    fn write_self<W: std::io::Write>(&self, f: &mut W, _: &ptree::Style) -> std::io::Result<()> {
        match self {
            Self::Unary {
                node_type,
                return_type,
                ..
            } => write!(f, "Unary[{} - {}]", node_type, return_type),
            Self::Binary {
                node_type,
                return_type,
                ..
            } => write!(f, "Bin[{} - {}]", node_type, return_type),
            Self::Constant {
                node_type, value, ..
            } => write!(f, "Const[{} - {}]", node_type, value),
            Self::Symbol {
                node_type,
                symbol_id,
                ..
            } => write!(f, "Sym[{} - ID:{}]", node_type, symbol_id),
            Self::Empty => write!(f, "[EMPTY]"),
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        match self {
            Self::Unary { child, .. } => {
                if let Some(node) = child {
                    Cow::from(vec![node.deref().clone()])
                } else {
                    Cow::from(vec![])
                }
            }
            Self::Binary { left, right, .. } => {
                let mut vec: Vec<Node> = vec![];
                if let Some(node) = left {
                    vec.push(node.deref().clone());
                }
                if let Some(node) = right {
                    vec.push(node.deref().clone());
                }
                Cow::from(vec)
            }
            Self::Constant { .. } => Cow::from(vec![]),
            Self::Symbol { .. } => Cow::from(vec![]),
            Self::Empty => Cow::from(vec![]),
        }
    }
}
