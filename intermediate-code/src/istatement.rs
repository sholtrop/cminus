use crate::{
    ioperand::IOperand,
    ioperator::{IOperator, IOperatorType},
};
use std::fmt;

pub struct IStatement {
    op_type: IOperatorType,
    operator: IOperator,
    operand1: Option<IOperand>,
    operand2: Option<IOperand>,
    returns: Option<IOperand>,
}

impl fmt::Display for IStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.op_type, self.operator)?;
        if let Some(operand1) = &self.operand1 {
            write!(f, " {}", operand1)?;
        }
        if let Some(operand2) = &self.operand2 {
            write!(f, ", {}", operand2)?;
        }
        if let Some(returns) = &self.returns {
            write!(f, " = {}", returns)?;
        }
        Ok(())
    }
}
