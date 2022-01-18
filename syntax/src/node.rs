use crate::{error::SyntaxBuilderError, id::SymbolId, symbol::ReturnType, visitor::SyntaxResult};
use core::fmt;
use std::{borrow::Cow, cell::RefCell, rc::Rc};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TESTING: bool = std::env::var("TESTING").map_or(false, |s| s != "0");
}

#[derive(PartialEq, Clone, Debug)]
pub enum NodeType {
    Unknown,
    Error,
    StatementList,
    Assignment,
    If,
    IfTargets,
    While,
    ArrayAccess,
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
                NodeType::ArrayAccess => "array_access",
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

impl NodeType {
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::Add
                | Self::Sub
                | Self::Or
                | Self::Mul
                | Self::Div
                | Self::IDiv
                | Self::Mod
                | Self::And
                | Self::Assignment
                | Self::RelEqual
                | Self::RelLT
                | Self::RelGT
                | Self::RelLTE
                | Self::RelGTE
                | Self::RelNotEqual
                | Self::Num
                | Self::Not
                | Self::SignPlus
                | Self::SignMinus
                | Self::Coercion
                | Self::FunctionCall
                | Self::Id
                | Self::ArrayAccess
        )
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum ConstantNodeValue {
    Uint8(u8),
    Int8(i8),
    Int(i32),
    Uint(u32),
}

impl std::ops::Add for ConstantNodeValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantNodeValue::Int(x), ConstantNodeValue::Int(y)) => {
                ConstantNodeValue::Int(i32::wrapping_add(x, y))
            }
            (ConstantNodeValue::Uint(x), ConstantNodeValue::Uint(y)) => {
                ConstantNodeValue::Uint(u32::wrapping_add(x, y))
            }

            (ConstantNodeValue::Int8(x), ConstantNodeValue::Int8(y)) => {
                ConstantNodeValue::Int8(i8::wrapping_add(x, y))
            }

            (ConstantNodeValue::Uint8(x), ConstantNodeValue::Uint8(y)) => {
                ConstantNodeValue::Uint8(u8::wrapping_add(x, y))
            }
            _ => unreachable!(
                "Cannot add two different ConstantNodeValue types: {:?} and {:?}",
                self, rhs
            ),
        }
    }
}

impl std::ops::Sub for ConstantNodeValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantNodeValue::Int(x), ConstantNodeValue::Int(y)) => {
                ConstantNodeValue::Int(i32::wrapping_sub(x, y))
            }
            (ConstantNodeValue::Uint(x), ConstantNodeValue::Uint(y)) => {
                ConstantNodeValue::Uint(u32::wrapping_sub(x, y))
            }

            (ConstantNodeValue::Int8(x), ConstantNodeValue::Int8(y)) => {
                ConstantNodeValue::Int8(i8::wrapping_sub(x, y))
            }

            (ConstantNodeValue::Uint8(x), ConstantNodeValue::Uint8(y)) => {
                ConstantNodeValue::Uint8(u8::wrapping_sub(x, y))
            }
            _ => unreachable!(
                "Cannot subtract two different ConstantNodeValue types: {:?} and {:?}",
                self, rhs
            ),
        }
    }
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
            }
        )
    }
}

impl From<ConstantNodeValue> for i64 {
    fn from(val: ConstantNodeValue) -> Self {
        match val {
            ConstantNodeValue::Int(v) => v.into(),
            ConstantNodeValue::Int8(v) => v.into(),
            ConstantNodeValue::Uint(v) => v.into(),
            ConstantNodeValue::Uint8(v) => v.into(),
        }
    }
}

pub struct PreorderIter {
    stack: Vec<SyntaxNodeBox>,
}

impl PreorderIter {
    pub fn new(root: SyntaxNodeBox) -> Self {
        Self { stack: vec![root] }
    }
}

impl Iterator for PreorderIter {
    type Item = SyntaxNodeBox;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.stack.pop() {
            let node = &*n.borrow();
            match node {
                SyntaxNode::Binary {
                    ref left,
                    ref right,
                    ..
                } => {
                    if let Some(right) = right {
                        // let r = right.clone();
                        self.stack.push(right.clone());
                    }
                    if let Some(left) = left {
                        self.stack.push(left.clone());
                    }
                }
                SyntaxNode::Unary {
                    child: Some(child), ..
                } => {
                    self.stack.push(child.clone());
                }
                _ => {}
            }
            Some(n.clone())
        } else {
            None
        }
    }
}

pub type SyntaxCell = RefCell<SyntaxNode>;
pub type SyntaxNodeBox = Rc<SyntaxCell>;
pub type SyntaxNodeChild = Option<SyntaxNodeBox>;

#[derive(PartialEq, Clone, Debug)]
pub enum SyntaxNode {
    Unary {
        node_type: NodeType,
        return_type: ReturnType,
        child: SyntaxNodeChild,
    },
    Binary {
        node_type: NodeType,
        return_type: ReturnType,
        left: SyntaxNodeChild,
        right: SyntaxNodeChild,
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
    Error,
    Empty,
}

impl SyntaxNode {
    pub fn create_error() -> Self {
        SyntaxNode::Error
    }

    pub fn create_child(node: SyntaxNode) -> SyntaxNodeChild {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn create_boxed(node: SyntaxNode) -> SyntaxNodeBox {
        Rc::new(RefCell::new(node))
    }

    /// Attempt to coerce [SyntaxNode] `from` to [ReturnType] `to`
    /// If the coercion is valid, will return a unary coercion [SyntaxNode] with the `from` node as its child.
    pub fn coerce(from: SyntaxNode, to: ReturnType) -> SyntaxResult {
        log::trace!("Coerce from {} to {}", from, to);
        let from_ret_t = from.return_type();
        if from_ret_t == ReturnType::Void {
            return Err("Expression must have a return value".into());
        }
        if from_ret_t == to {
            Ok(from)
        }
        // ReturnTypes have a defined partial ordering for coercion.
        // Any type can be coerced to bool or error.
        else if matches!(to, ReturnType::Bool | ReturnType::Error) || from_ret_t < to {
            Ok(SyntaxNode::Unary {
                child: SyntaxNode::create_child(from),
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
            SyntaxNode::Empty => ReturnType::Void,
            SyntaxNode::Error => ReturnType::Error,
        }
    }

    pub fn node_type(&self) -> NodeType {
        match self {
            SyntaxNode::Unary { node_type, .. }
            | SyntaxNode::Binary { node_type, .. }
            | SyntaxNode::Constant { node_type, .. }
            | SyntaxNode::Symbol { node_type, .. } => node_type.clone(),
            SyntaxNode::Empty => NodeType::Empty,
            SyntaxNode::Error => NodeType::Error,
        }
    }

    pub fn symbol_id(&self) -> SymbolId {
        if let SyntaxNode::Symbol { symbol_id, .. } = self {
            *symbol_id
        } else {
            panic!("SyntaxNode::symbol_id called on non-symbol Node")
        }
    }

    pub fn preorder(root: &SyntaxNodeBox) -> PreorderIter {
        PreorderIter::new(root.clone())
    }

    /*
    assign => 0,
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

    pub fn get_binary_children(&self) -> (SyntaxNodeChild, SyntaxNodeChild) {
        if let SyntaxNode::Binary { left, right, .. } = self {
            (left.as_ref().cloned(), right.as_ref().cloned())
        } else {
            panic!("Node {} was not binary", self);
        }
    }

    /// Returns left and right child of a binary node.
    /// Panics if the node is not binary or if either of the two children is [None].
    pub fn get_both_binary_children(&self) -> (SyntaxNodeBox, SyntaxNodeBox) {
        if let SyntaxNode::Binary { left, right, .. } = self {
            (
                left.as_ref().unwrap().clone(),
                right.as_ref().unwrap().clone(),
            )
        } else {
            panic!("Node {} was not binary", self);
        }
    }

    pub fn get_unary_child(&self) -> SyntaxNodeChild {
        if let SyntaxNode::Unary { child, .. } = self {
            child.as_ref().cloned()
        } else {
            panic!("Node {} was not unary", self);
        }
    }

    /// Get a constant node's value
    pub fn get_number(&self) -> ConstantNodeValue {
        if let SyntaxNode::Constant { value, .. } = self {
            *value
        } else {
            panic!("Node {} was not a number", self);
        }
    }

    pub fn is_binop(&self) -> bool {
        matches!(
            self.node_type(),
            NodeType::Add
                | NodeType::And
                | NodeType::Or
                | NodeType::Sub
                | NodeType::Mul
                | NodeType::Mod
                | NodeType::Div
                | NodeType::RelEqual
                | NodeType::RelNotEqual
                | NodeType::RelGT
                | NodeType::RelGTE
                | NodeType::RelLT
                | NodeType::RelLTE
                | NodeType::Assignment
        )
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                symbol_id,
                return_type,
                ..
            } => {
                if *TESTING {
                    write!(f, "symbol - {}", return_type)
                } else {
                    write!(f, "symbol - Sym:{}", symbol_id)
                }
            }
            Self::Empty => write!(f, "[EMPTY]"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

impl ptree::TreeItem for SyntaxNode {
    type Child = Self;
    fn write_self<W: std::io::Write>(&self, f: &mut W, _: &ptree::Style) -> std::io::Result<()> {
        write!(f, "{}", self)
    }
    fn children(&self) -> Cow<[Self::Child]> {
        match self {
            Self::Unary { child, .. } => {
                if let Some(node) = child {
                    let x = (*node.as_ref()).borrow().clone();
                    Cow::from(vec![x])
                } else {
                    Cow::from(vec![])
                }
            }
            Self::Binary { left, right, .. } => {
                let mut vec: Vec<SyntaxNode> = vec![];
                if let Some(node) = left {
                    let x = (*node.as_ref()).borrow().clone();
                    vec.push(x);
                }
                if let Some(node) = right {
                    let x = (*node.as_ref()).borrow().clone();
                    vec.push(x);
                }
                Cow::from(vec)
            }
            Self::Constant { .. } => Cow::from(vec![]),
            Self::Symbol { .. } => Cow::from(vec![]),
            Self::Empty | Self::Error => Cow::from(vec![]),
        }
    }
}

impl From<SyntaxBuilderError> for SyntaxNode {
    fn from(_: SyntaxBuilderError) -> Self {
        Self::Error
    }
}
