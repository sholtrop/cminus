use general::tree::{ChildPosition, TreeIndex};

use crate::symbol::ReturnType;
use core::fmt;

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub enum SyntaxNode {
    Unary {
        node_type: NodeType,
        return_type: ReturnType,
        child: Option<TreeIndex>,
    },
    Binary {
        node_type: NodeType,
        return_type: ReturnType,
        left: Option<TreeIndex>,
        right: Option<TreeIndex>,
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

impl general::tree::TreeItem for SyntaxNode {
    fn get_child(&self, pos: &general::tree::ChildPosition) -> Option<general::tree::TreeIndex> {
        match self {
            &Self::Binary { left, right, .. } => {
                if let ChildPosition::Left = pos {
                    left
                } else {
                    right
                }
            }
            &Self::Unary { child, .. } => child,
            _ => {
                panic!("Cannot get child of SyntaxNode {:?}", self)
            }
        }
    }

    fn set_child(
        &mut self,
        pos: &general::tree::ChildPosition,
        new_child: general::tree::TreeIndex,
    ) -> Option<general::tree::TreeIndex> {
        match *self {
            Self::Binary {
                ref mut left,
                ref mut right,
                ..
            } => {
                let old: Option<TreeIndex>;
                if let ChildPosition::Left = pos {
                    old = *left;
                    *left = Some(new_child);
                } else {
                    old = *right;
                    *right = Some(new_child);
                }
                old
            }
            Self::Unary { ref mut child, .. } => {
                let old = *child;
                *child = Some(new_child);
                old
            }
            _ => panic!("Cannot set child of SyntaxNode {:?}", self),
        }
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyntaxNode::Binary { node_type, .. } => node_type.to_string(),
                SyntaxNode::Constant { node_type, .. } => node_type.to_string(),
                SyntaxNode::Empty => "Empty".to_string(),
                SyntaxNode::Symbol { node_type, .. } => node_type.to_string(),
                SyntaxNode::Unary { node_type, .. } => node_type.to_string(),
            }
        )
    }
}
