use std::fmt;

use syntax::ConstantNodeValue;
use syntax::NodeType;
use syntax::ReturnType;

#[derive(PartialEq)]
pub enum IOperatorType {
    Void,
    Byte,
    Word,
    Double,
    Quad,
}

impl fmt::Display for IOperatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Void => "",
                Self::Byte => "b",
                Self::Word => "w",
                Self::Double => "l",
                Self::Quad => "q",
            }
        )
    }
}

impl From<ReturnType> for IOperatorType {
    fn from(rt: ReturnType) -> Self {
        match rt {
            ReturnType::Bool | ReturnType::Uint8 | ReturnType::Int8 => Self::Byte,
            ReturnType::Uint | ReturnType::Int => Self::Double,
            ReturnType::Real
            | ReturnType::Int8Array
            | ReturnType::Uint8Array
            | ReturnType::UintArray
            | ReturnType::IntArray => Self::Quad,
            _ => unreachable!("Cannot convert {} to IOperatorType", rt),
        }
    }
}

pub enum IOperator {
    Unknown,
    Func,
    Return,
    Param,
    FuncCall,
    Label,
    Goto,
    Assign,
    Larray,
    Rarray,

    // Branching
    Je,  // ==
    Jne, // !=
    Jb,  // < unsigned
    JL,  // < signed
    Jnb, // >= unsigned
    Jge, // >= signed
    Jbe, // <= unsigned
    Jle, // <= signed
    Ja,  // > unsigned
    Jg,  // > signed
    Jnz, // not zero
    Jz,  // zero

    // Conditional set
    SetE,  // == set
    SetNE, // != set
    SetG,  // > signed set
    SetGE, // >= signed set
    SetL,  // < signed set
    SetLE, // <= signed set
    SetA,  // > unsigned set
    SetNB, // >= unsigned set
    SetB,  // < unsigned set
    SetBE, // <= unsigned set

    // Binary arithmetic operators
    Add,  // Addition
    Sub,  // Subtraction
    Mul,  // Multiplication
    Div,  // Unsigned Division
    IDiv, // Signed Division
    Mod,  // Modulo
    Imod, // Signed Modulo
    And,  // AND operation
    Or,   // OR operation

    // Unary arithmetic operators
    Not,   // !
    Minus, // -

    // Coercion
    Coerce,
}

impl fmt::Display for IOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Unknown => "unknown",
                Self::Func => "FUNC",
                Self::Return => "RETURN",
                Self::Param => "PARAM",
                Self::FuncCall => "CALL_FUNC",
                Self::Label => "@LABEL",
                Self::Goto => "GOTO",
                Self::Assign => "ASSIGN",
                Self::Larray => "LARRAY",
                Self::Rarray => "RARRAY",
                Self::Je => "JUMP_EQUAL",
                Self::Jne => "JUMP_NOT_EQUAL",
                Self::Jb => "JUMP_BELOW",
                Self::JL => "JUMP_LESS",
                Self::Jnb => "JUMP_NOT_BELOW",
                Self::Jge => "JUMP_GREATER_EQUAL",
                Self::Jbe => "JUMP_BELOW_EQUAL",
                Self::Jle => "JUMP_LESS_EQUAL",
                Self::Ja => "JUMP_ABOVE",
                Self::Jg => "JUMP_GREATER",
                Self::Jnz => "JUMP_TRUE",
                Self::Jz => "JUMP_FALSE",
                Self::SetE => "SET_IF_EQUAL",
                Self::SetNE => "SET_IF_NOT_EQUAL",
                Self::SetG => "SET_IF_GREATER",
                Self::SetGE => "SET_IF_GREATER_EQUAL",
                Self::SetL => "SET_IF_LESS",
                Self::SetLE => "SET_IF_LESS_EQUAL",
                Self::SetA => "SET_IF_ABOVE",
                Self::SetNB => "SET_IF_NOT_BELOW",
                Self::SetB => "SET_IF_BELOW",
                Self::SetBE => "SET_IF_BELOW_EQUAL",
                Self::Add => "ADD",
                Self::Sub => "SUB",
                Self::Mul => "MUL",
                Self::Div => "DIV",
                Self::IDiv => "SIGNED_DIV",
                Self::Mod => "MOD",
                Self::Imod => "SIGNED_MOD",
                Self::And => "AND",
                Self::Or => "OR",
                Self::Not => "NOT",
                Self::Minus => "UNARY_MINUS",
                Self::Coerce => "COERCE",
            }
        )
    }
}

impl From<NodeType> for IOperator {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Add => Self::Add,
            NodeType::Sub => Self::Sub,
            NodeType::Mul => Self::Mul,
            NodeType::Div => Self::Div,
            NodeType::Assignment => Self::Assign,
            NodeType::Unknown => Self::Unknown,
            _ => unreachable!("Cannot convert {} to IOperator", node_type),
        }
    }
}
