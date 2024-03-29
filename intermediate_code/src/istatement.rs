use syntax::{ReturnType, SymbolId, BUILTIN_IDS};

use crate::{
    ioperand::IOperand,
    ioperator::{IOperator, IOperatorSize},
};
use std::fmt;

#[derive(Clone)]
pub struct IStatement {
    pub op_type: IOperatorSize,
    pub operator: IOperator,
    pub operand1: Option<IOperand>,
    pub operand2: Option<IOperand>,
    pub ret_target: Option<IOperand>,
}

impl IStatement {
    pub fn make_label(label_id: SymbolId) -> Self {
        Self {
            op_type: IOperatorSize::Void,
            operator: IOperator::Label,
            operand1: Some(IOperand::Symbol {
                id: label_id,
                ret_type: ReturnType::Label,
            }),
            operand2: None,
            ret_target: None,
        }
    }

    pub fn make_goto(target_id: SymbolId) -> Self {
        Self {
            op_type: IOperatorSize::Void,
            operator: IOperator::Goto,
            operand1: Some(IOperand::Symbol {
                id: target_id,
                ret_type: ReturnType::Label,
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

    pub fn is_unconditional_jump(&self) -> bool {
        self.operator == IOperator::Goto
    }

    pub fn is_conditional_jump(&self) -> bool {
        matches!(
            self.operator,
            IOperator::Jl
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
        )
    }

    pub fn is_jump(&self) -> bool {
        self.is_conditional_jump()
            || self.is_unconditional_jump()
            || self.operator == IOperator::Return
    }

    pub fn is_non_builtin_call(&self) -> bool {
        self.is_call() && !BUILTIN_IDS.contains(&self.label_id().0)
    }

    pub fn is_call(&self) -> bool {
        self.operator == IOperator::FuncCall
    }

    pub fn is_recursive_call(&self, current_func: &SymbolId) -> bool {
        self.is_call() && self.label_id() == *current_func
    }

    pub fn is_return(&self) -> bool {
        self.operator == IOperator::Return
    }

    pub fn label_id(&self) -> SymbolId {
        if self.is_unconditional_jump() || self.is_label() || self.is_func() || self.is_call() {
            self.operand1.as_ref().unwrap().id()
        } else if self.is_conditional_jump() {
            self.ret_target.as_ref().unwrap().id()
        } else {
            unreachable!()
        }
    }

    pub fn is_assign(&self) -> bool {
        self.operator == IOperator::Assign
    }

    pub fn is_binop(&self) -> bool {
        matches!(
            self.operator,
            IOperator::Add
                | IOperator::And
                | IOperator::Div
                | IOperator::Mul
                | IOperator::Or
                | IOperator::Sub
                | IOperator::Mod
                | IOperator::IDiv
                | IOperator::Imod
        )
    }

    pub fn is_param(&self) -> bool {
        self.operator == IOperator::Param
    }

    pub fn get_triple(&self) -> (&IOperand, &IOperand, &IOperand) {
        (
            self.operand1.as_ref().unwrap(),
            self.operand2.as_ref().unwrap(),
            self.ret_target.as_ref().unwrap(),
        )
    }
}

impl fmt::Display for IStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.op_type,
            if self.op_type == IOperatorSize::Void {
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
