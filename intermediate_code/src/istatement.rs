use syntax::{ReturnType, SymbolId};

use crate::{
    ioperand::IOperand,
    ioperator::{IOperator, IOperatorType},
};
use std::fmt;

pub struct IStatement {
    pub op_type: IOperatorType,
    pub operator: IOperator,
    pub operand1: Option<IOperand>,
    pub operand2: Option<IOperand>,
    pub ret_target: Option<IOperand>,
}

impl IStatement {
    pub fn make_label(label_id: SymbolId) -> Self {
        Self {
            op_type: IOperatorType::Void,
            operator: IOperator::Label,
            operand1: Some(IOperand::Symbol {
                id: label_id,
                ret_type: ReturnType::Void,
            }),
            operand2: None,
            ret_target: None,
        }
    }

    pub fn make_goto(target_id: SymbolId) -> Self {
        Self {
            op_type: IOperatorType::Void,
            operator: IOperator::Goto,
            operand1: Some(IOperand::Symbol {
                id: target_id,
                ret_type: ReturnType::Void,
            }),
            operand2: None,
            ret_target: None,
        }
    }

    pub fn is_label(&self) -> bool {
        self.operator == IOperator::Label
    }

    pub fn is_func(&self) -> bool {
        self.operator == IOperator::Func
    }

    pub fn is_jump(&self) -> bool {
        matches!(
            self.operator,
            IOperator::Goto
                | IOperator::Jl
                | IOperator::Ja
                | IOperator::Jae
                | IOperator::Jb
                | IOperator::Jbe
                | IOperator::Jg
                | IOperator::Jge
                | IOperator::Jle
                | IOperator::Jne
                | IOperator::Je
                | IOperator::Jz
                | IOperator::Jnz
                | IOperator::Return
        )
    }

    pub fn is_call(&self) -> bool {
        self.operator == IOperator::FuncCall
    }

    pub fn label_id(&self) -> SymbolId {
        self.operand1.as_ref().unwrap().id()
    }
}

impl fmt::Display for IStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.operator == IOperator::Func {
            writeln!(f)?;
        }
        write!(
            f,
            "{}{}{}",
            self.op_type,
            if self.op_type == IOperatorType::Void {
                ""
            } else {
                ":"
            },
            self.operator
        )?;
        if let Some(operand1) = &self.operand1 {
            write!(f, " [{}]", operand1)?;
        }

        if let Some(operand2) = &self.operand2 {
            write!(f, " [{}]", operand2)?;
        }

        if let Some(returns) = &self.ret_target {
            write!(f, " -> [{}]", returns)?;
        }
        Ok(())
    }
}
