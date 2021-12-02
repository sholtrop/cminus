use crate::{error::SyntaxBuilderError, id::SymbolId, symbol::ReturnType, visitor::SyntaxResult};
use core::fmt;
use ptree::TreeItem;
use std::{borrow::Cow, ops::Deref};

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
                NodeType::Id => "sym_id",
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
    Uint(u32),
    ErrorMessage(String),
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
                Self::Uint(v) => v.to_string(),
                Self::ErrorMessage(v) => v.to_string(),
            }
        )
    }
}

impl From<ConstantNodeValue> for i64 {
    fn from(val: ConstantNodeValue) -> Self {
        match val {
            ConstantNodeValue::ErrorMessage(msg) => panic!("Node had error message: {}", msg),
            ConstantNodeValue::Int(v) => v.into(),
            ConstantNodeValue::Int8(v) => v.into(),
            ConstantNodeValue::Uint(v) => v.into(),
            ConstantNodeValue::Uint8(v) => v.into(),
        }
    }
}

pub struct PreorderIter<'a> {
    stack: Vec<&'a SyntaxNode>,
}

impl<'a> PreorderIter<'a> {
    pub fn new(root: &'a SyntaxNode) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for PreorderIter<'a> {
    type Item = &'a SyntaxNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.stack.pop() {
            match *n {
                SyntaxNode::Binary {
                    ref left,
                    ref right,
                    ..
                } => {
                    if let Some(left) = left {
                        self.stack.push(left.as_ref());
                    }
                    if let Some(right) = right {
                        self.stack.push(right.as_ref());
                    }
                }
                SyntaxNode::Unary {
                    child: Some(ref child),
                    ..
                } => {
                    self.stack.push(child.as_ref());
                }
                _ => {}
            }
            Some(n)
        } else {
            None
        }
    }
}

pub type SyntaxNodeBox = Option<Box<SyntaxNode>>;

#[derive(PartialEq, Clone, Debug)]
pub enum SyntaxNode {
    Unary {
        node_type: NodeType,
        return_type: ReturnType,
        child: SyntaxNodeBox,
    },
    Binary {
        node_type: NodeType,
        return_type: ReturnType,
        left: SyntaxNodeBox,
        right: SyntaxNodeBox,
    },
    Constant {
        node_type: NodeType,
        return_type: ReturnType,
        value: ConstantNodeValue,
    },
    Symbol {
        node_type: NodeType,
        return_type: ReturnType,
        symbol_id: SymbolId,
    },
    Empty,
}

impl SyntaxNode {
    pub fn create_error(err: impl ToString) -> Self {
        SyntaxNode::Constant {
            node_type: NodeType::Error,
            return_type: ReturnType::Error,
            value: ConstantNodeValue::ErrorMessage(err.to_string()),
        }
    }

    /// Attempt to coerce [SyntaxNode] `from` to [ReturnType] `to`
    /// If the coercion is valid, will return a unary coercion [SyntaxNode] with the `from` node as its child.
    pub fn coerce(from: SyntaxNode, to: ReturnType) -> SyntaxResult {
        let from_ret_t = from.return_type();
        assert_ne!(from_ret_t, to);
        if to == ReturnType::Bool {
            Ok(SyntaxNode::Unary {
                child: Some(Box::new(from)),
                node_type: NodeType::Coercion,
                return_type: to,
            })
        }
        // ReturnTypes have a defined partial ordering for coercion
        else if from_ret_t < to {
            Ok(SyntaxNode::Unary {
                child: Some(Box::new(from)),
                return_type: to,
                node_type: NodeType::Coercion,
            })
        } else {
            Err(SyntaxBuilderError(format!(
                "Cannot coerce {} to {}",
                from_ret_t, to
            )))
        }
    }

    pub fn return_type(&self) -> ReturnType {
        match self {
            SyntaxNode::Unary { return_type, .. }
            | SyntaxNode::Binary { return_type, .. }
            | SyntaxNode::Constant { return_type, .. }
            | SyntaxNode::Symbol { return_type, .. } => *return_type,
            SyntaxNode::Empty => ReturnType::Error,
        }
    }

    pub fn node_type(&self) -> NodeType {
        match self {
            SyntaxNode::Unary { node_type, .. }
            | SyntaxNode::Binary { node_type, .. }
            | SyntaxNode::Constant { node_type, .. }
            | SyntaxNode::Symbol { node_type, .. } => node_type.clone(),
            SyntaxNode::Empty => NodeType::Error,
        }
    }

    pub fn preorder(&self) -> PreorderIter {
        PreorderIter::new(self)
    }

    /*        assign => 0,
    or | and => 1,
    gt | gte | lt | lte => 2,
    eq | neq => 3,
    add | sub => 4,
    mul | div | modulo => 5,
    _ => 6, */
    pub fn precedence(&self) -> Result<u8, SyntaxBuilderError> {
        use NodeType::*;

        if let Self::Binary { node_type, .. } = self {
            match node_type {
                Assignment => Ok(0),
                Or | And => Ok(1),
                RelGT | RelGTE | RelLT | RelLTE => Ok(2),
                RelEqual | RelNotEqual => Ok(3),
                Add | Sub => Ok(4),
                Mul | Div | Mod => Ok(5),
                _ => Err(SyntaxBuilderError(format!(
                    "Node {} is not an infix operator",
                    self
                ))),
            }
        } else {
            Err(SyntaxBuilderError(format!(
                "Node {} is not an infix operator",
                self
            )))
        }
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxNode::Binary { node_type, .. } => {
                writeln!(f, "bin:{}", node_type.to_string())
            }
            SyntaxNode::Constant { node_type, .. } => {
                writeln!(f, "const:{}", node_type.to_string())
            }
            SyntaxNode::Empty => writeln!(f, "Empty"),
            SyntaxNode::Symbol { node_type, .. } => {
                writeln!(f, "sym:{}", node_type.to_string())
            }
            SyntaxNode::Unary { node_type, .. } => writeln!(f, "un:{}", node_type.to_string()),
        }
    }
}

impl ptree::TreeItem for SyntaxNode {
    type Child = Self;
    fn write_self<W: std::io::Write>(&self, f: &mut W, _: &ptree::Style) -> std::io::Result<()> {
        match self {
            Self::Unary {
                node_type,
                return_type,
                ..
            } => write!(f, "{} - {}", node_type, return_type),
            Self::Binary {
                node_type,
                return_type,
                ..
            } => write!(f, "{} - {}", node_type, return_type),
            Self::Constant {
                node_type, value, ..
            } => write!(f, "{} - {}", node_type, value),
            Self::Symbol {
                node_type,
                symbol_id,
                ..
            } => write!(f, "{} - Sym:{}", node_type, symbol_id),
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
                let mut vec: Vec<SyntaxNode> = vec![];
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

impl From<SyntaxBuilderError> for SyntaxNode {
    fn from(err: SyntaxBuilderError) -> Self {
        Self::Constant {
            node_type: NodeType::Error,
            return_type: ReturnType::Error,
            value: ConstantNodeValue::ErrorMessage(err.to_string()),
        }
    }
}
