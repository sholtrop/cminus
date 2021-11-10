use core::fmt;

use crate::symbol::ReturnType;

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum ConstantNodeValue {
    Uint8(u8),
    Int8(i8),
    Int(i32),
    Unsigned(u32),
}

#[derive(PartialEq)]
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
}

// pub struct UnaryNode {
//     node: Node,
// }

// pub struct BinaryNode {
//     node: Node,
// }

// pub struct ConstantNode {}

// pub struct SymbolNode {}
