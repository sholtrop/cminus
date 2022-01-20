use crate::icode::IntermediateCode;
use crate::ioperand::IOperand;
use crate::ioperator::{IOperator, IOperatorSize};
use crate::istatement::IStatement;
use syntax::{ConstantNodeValue, SymbolId, SyntaxNode, SyntaxNodeBox};
use syntax::{
    NodeType::{self, *},
    ReturnType,
};

pub struct IVisitor<'a> {
    table: &'a mut syntax::SymbolTable,
    icode: IntermediateCode,
    func_stack: Vec<SymbolId>,
    // label_counter: usize,
    // temp_counter: usize,
}

impl<'a> IVisitor<'a> {
    pub fn new(table: &'a mut syntax::SymbolTable) -> Self {
        Self {
            table,
            icode: IntermediateCode::new(),
            func_stack: vec![],
            // label_counter: 0,
            // temp_counter: 0,
        }
    }
    pub fn result(self) -> IntermediateCode {
        self.icode
    }

    fn accept(&mut self, node: SyntaxNodeBox) {
        log::trace!("{}", *node.borrow());
        let ntype = (*node.borrow()).node_type();
        if ntype.is_expression() {
            self.accept_expression(node);
            return;
        }
        match ntype {
            Empty => {}
            StatementList => {
                let (l, r) = (*node.borrow()).get_binary_children();
                self.accept(l.expect("Left of StatementList was None"));
                if let Some(r) = r {
                    self.accept(r);
                }
            }
            If => {
                let (cond, targets) = (*node.borrow()).get_both_binary_children();
                // let t = (*targets.borrow());
                // if without else
                let (if_body, else_body) = if (*targets.borrow()).node_type() != IfTargets {
                    (targets, None)
                }
                // if with else
                else {
                    let (if_body, else_body) = (*targets.borrow()).get_both_binary_children();
                    (if_body, Some(else_body))
                };
                self.visit_if_else(cond, if_body, else_body);
            }
            While => {
                let (cond, body) = (*node.borrow()).get_both_binary_children();
                self.visit_while(cond, body);
            }
            Return => {
                let ret_child = (*node.borrow()).get_unary_child();
                self.visit_return(ret_child);
            }
            _ => {
                unimplemented!("{:?}", (*node.borrow()).node_type());
            }
        }
    }

    // TODO:
    // implement short-circuiting, i.e.
    // int x = 0 && func();
    // should never call `func`
    fn accept_expression(&mut self, exp: SyntaxNodeBox) -> IOperand {
        let ntype = (*exp.borrow()).node_type();
        log::trace!("Expression: {}", ntype);
        match ntype {
            Assignment => {
                let (l, r) = (*exp.borrow()).get_both_binary_children();
                self.visit_assignment(l, r)
            }
            Add | Sub | Mul | Div | Mod | And | Or | RelEqual | RelNotEqual | RelGT | RelGTE
            | RelLT | RelLTE => {
                let (l, r) = (*exp.borrow()).get_both_binary_children();
                let ret_type = if ntype.is_rel_expression() {
                    ReturnType::Bool
                } else {
                    (*l.borrow()).return_type()
                };
                let operator = if ret_type.is_unsigned() {
                    IOperator::from(ntype).to_unsigned()
                } else {
                    IOperator::from(ntype)
                };
                let l_expr = self.accept_expression(l);
                let mut r_expr = self.accept_expression(r);

                // if matches!(ntype, Div | Mod) {
                //     // Div and Mod require the divisor to be in a register
                //     if let IOperand::Immediate { ret_type, .. } = r_expr {
                //         let temp = self.make_temp(ret_type);
                //         r_expr = IOperand::Symbol { id: temp, ret_type };
                //     }
                // }

                let ret = self.make_temp(ret_type);
                let ret_target = IOperand::Symbol { id: ret, ret_type };
                self.icode.append_statement(IStatement {
                    op_type: ret_type.into(),
                    operator,
                    operand1: Some(l_expr),
                    operand2: Some(r_expr),
                    ret_target: Some(ret_target.clone()),
                });
                ret_target
            }
            Coercion => {
                let child = (*exp.borrow()).get_unary_child().unwrap();
                let ret_type = (*exp.borrow()).return_type();
                let temp = self.make_temp(ret_type);
                let precoercion = self.accept_expression(child);
                let postcoercion = IOperand::Symbol { id: temp, ret_type };
                self.icode.append_statement(IStatement {
                    op_type: ret_type.into(),
                    operator: IOperator::Coerce,
                    operand1: Some(precoercion),
                    operand2: None,
                    ret_target: Some(postcoercion.clone()),
                });
                postcoercion
            }
            FunctionCall => {
                let (func, args) = (*exp.borrow()).get_binary_children();
                let func = func.unwrap();
                self.visit_func_call(func, args)
            }
            ArrayAccess => self.visit_array_access(exp),
            Num => {
                let num = (*exp.borrow()).get_number();
                IOperand::Immediate {
                    value: num,
                    ret_type: (*exp.borrow()).return_type(),
                }
            }
            Id => {
                if let SyntaxNode::Symbol {
                    symbol_id,
                    return_type,
                    ..
                } = *exp.borrow()
                {
                    IOperand::Symbol {
                        id: symbol_id,
                        ret_type: return_type,
                    }
                } else {
                    unreachable!()
                }
            }
            Not => {
                let child = (*exp.borrow()).get_unary_child().unwrap();
                let child_exp = self.accept_expression(child);
                let ret_target = self.make_temp(ReturnType::Bool);
                let ret_target = IOperand::Symbol {
                    id: ret_target,
                    ret_type: ReturnType::Bool,
                };
                self.icode.append_statement(IStatement {
                    op_type: IOperatorSize::Byte,
                    operator: IOperator::Not,
                    operand1: Some(child_exp),
                    operand2: None,
                    ret_target: Some(ret_target.clone()),
                });
                ret_target
            }
            SignMinus => {
                let child = (*exp.borrow()).get_unary_child().unwrap();
                let ret_type = (*child.borrow()).return_type();
                let child_exp = self.accept_expression(child);
                let ret_target = self.make_temp(ret_type);
                let ret_target = IOperand::Symbol {
                    id: ret_target,
                    ret_type,
                };
                self.icode.append_statement(IStatement {
                    op_type: ret_type.into(),
                    operator: IOperator::Minus,
                    operand1: Some(child_exp),
                    operand2: None,
                    ret_target: Some(ret_target.clone()),
                });
                ret_target
            }
            SignPlus => {
                let child = (*exp.borrow()).get_unary_child().unwrap();
                self.accept_expression(child)
            }
            _ => unreachable!("Node `{}` is not (part of) an expression", (*exp.borrow())),
        }
    }

    fn accept_cond_expression(&mut self, exp: SyntaxNodeBox) -> (IOperator, IOperand, IOperand) {
        let ntype = (*exp.borrow()).node_type();
        if ntype.is_rel_expression() {
            let (l, r) = (*exp.borrow()).get_both_binary_children();
            let l_expr = self.accept_expression(l);
            let r_expr = self.accept_expression(r);
            let op = IOperator::from(ntype).to_jump();
            log::debug!("Condop: {} | ntype: {}", op, ntype);
            (op, l_expr, r_expr)
        } else if ntype == NodeType::Coercion {
            let l = (*exp.borrow()).get_unary_child().unwrap();
            let expr = self.accept_expression(l);
            (
                IOperator::Jne,
                expr,
                IOperand::Immediate {
                    value: ConstantNodeValue::from(0),
                    ret_type: ReturnType::Bool,
                },
            )
        } else if ntype == NodeType::Num {
            let num = self.accept_expression(exp);
            (
                IOperator::Je,
                num,
                IOperand::Immediate {
                    value: ConstantNodeValue::from(0),
                    ret_type: ReturnType::Bool,
                },
            )
        } else {
            unreachable!(
                "accept_cond_expression only accept relational operators, coercions or num, got {}",
                ntype
            );
        }
    }

    pub fn visit_function(&mut self, func: SyntaxNodeBox, func_id: SymbolId) {
        if func_id.is_builtin() {
            return;
        }
        let name = self.table.get_symbol(&func_id).unwrap().name.clone();
        log::trace!("Visiting function {}", name);
        self.func_stack.push(func_id);
        let label_istmt = IOperand::Symbol {
            id: func_id,
            ret_type: ReturnType::Void,
        };
        self.icode.append_statement(IStatement {
            op_type: IOperatorSize::Void,
            operator: IOperator::Func,
            operand1: Some(label_istmt),
            operand2: None,
            ret_target: None,
        });
        self.accept(func.clone());
        let last_stmt = self.icode.get_last_statement();
        if !last_stmt.is_jump() && !last_stmt.is_recursive_call(&func_id) {
            self.add_implicit_return((*func.borrow()).return_type());
        }
    }

    fn current_func(&self) -> SymbolId {
        *self.func_stack.last().unwrap()
    }

    fn add_implicit_return(&mut self, ret_type: ReturnType) {
        self.icode.append_statement(IStatement {
            operator: IOperator::Return,
            op_type: ret_type.into(),
            operand1: None,
            operand2: None,
            ret_target: None,
        })
    }

    fn visit_assignment(&mut self, l_var: SyntaxNodeBox, r_expr: SyntaxNodeBox) -> IOperand {
        // let l_var = *l_var.borrow();
        // let r_expr = *r_expr.borrow();
        let common_ret = (*l_var.borrow()).return_type().to_base_type();
        let ret_target = if (*l_var.borrow()).node_type() == NodeType::ArrayAccess {
            self.visit_array_access(l_var)
        } else {
            IOperand::Symbol {
                id: (*l_var.borrow()).symbol_id(),
                ret_type: common_ret,
            }
        };
        let r_expr = self.accept_expression(r_expr);
        self.icode.append_statement(IStatement {
            op_type: common_ret.into(),
            operator: IOperator::Assign,
            operand1: None,
            operand2: Some(r_expr),
            ret_target: Some(ret_target.clone()),
        });
        ret_target
    }

    fn visit_if_else(
        &mut self,
        cond: SyntaxNodeBox,
        if_branch: SyntaxNodeBox,
        else_branch: Option<SyntaxNodeBox>,
    ) {
        let (op, l, r) = self.accept_cond_expression(cond);
        let else_label = self.make_label();
        // Check condition, jump to else-label if condition was false
        self.icode.append_statement(IStatement {
            op_type: IOperatorSize::Void,
            operator: op,
            operand1: Some(l),
            operand2: Some(r),
            ret_target: Some(IOperand::Symbol {
                id: else_label,
                ret_type: ReturnType::Void,
            }),
        });
        // If-body
        self.accept(if_branch);
        if let Some(else_branch) = else_branch {
            let end_else_label = self.make_label();
            // jump over the else-body if condition was true
            self.icode.insert_statement(
                IStatement::make_goto(end_else_label),
                self.icode.n_statements().into(),
            );
            // else-label
            self.icode
                .append_statement(IStatement::make_label(else_label));
            // else-body
            self.accept(else_branch);
            // end-else label
            self.icode
                .append_statement(IStatement::make_label(end_else_label));
        } else {
            // passed if-body label
            self.icode
                .append_statement(IStatement::make_label(else_label));
        }
    }

    fn visit_while(&mut self, cond: SyntaxNodeBox, body: SyntaxNodeBox) {
        let loop_cond = self.make_label();
        let loop_body = self.make_label();
        self.icode
            .append_statement(IStatement::make_goto(loop_cond));
        self.icode
            .append_statement(IStatement::make_label(loop_body));
        self.accept(body);
        self.icode
            .append_statement(IStatement::make_label(loop_cond));
        let (op, l, r) = self.accept_cond_expression(cond);
        self.icode.append_statement(IStatement {
            op_type: IOperatorSize::Void,
            operator: op,
            operand1: Some(l),
            operand2: Some(r),
            ret_target: Some(IOperand::Symbol {
                id: loop_body,
                ret_type: ReturnType::Void,
            }),
        });
    }

    fn visit_func_call(&mut self, func: SyntaxNodeBox, args: Option<SyntaxNodeBox>) -> IOperand {
        let func_id = (*func.borrow()).symbol_id();
        let ret_type = self.table.get_symbol(&func_id).unwrap().return_type;
        let ret_temp = self.make_temp(ret_type);
        let ret_temp = IOperand::Symbol {
            id: ret_temp,
            ret_type,
        };
        if let Some(args) = args {
            self.visit_expr_list(args);
        }
        self.icode.append_statement(IStatement {
            op_type: ret_type.into(),
            operator: IOperator::FuncCall,
            operand1: Some(IOperand::Symbol {
                id: func_id,
                ret_type,
            }),
            operand2: None,
            ret_target: Some(ret_temp.clone()),
        });
        ret_temp
    }

    fn visit_expr_list(&mut self, expr_list: SyntaxNodeBox) {
        let (current_exp_node, next) = (*expr_list.borrow()).get_binary_children();
        if let Some(ref exp_node) = current_exp_node {
            let exp = self.accept_expression(exp_node.clone());
            self.icode.append_statement(IStatement {
                op_type: (*exp_node.borrow()).return_type().into(),
                operator: IOperator::Param,
                operand1: Some(exp),
                operand2: None,
                ret_target: None,
            });
            if let Some(next) = next {
                self.visit_expr_list(next);
            }
        }
    }

    fn visit_return(&mut self, ret: Option<SyntaxNodeBox>) {
        let ret_exp = ret.map(|r| self.accept_expression(r));
        self.icode.append_statement(IStatement {
            op_type: IOperatorSize::Void,
            operator: IOperator::Return,
            operand1: ret_exp,
            operand2: None,
            ret_target: None,
        });
    }

    fn visit_array_access(&mut self, node: SyntaxNodeBox) -> IOperand {
        // let n = (*node.borrow());
        let (array, access) = (*node.borrow()).get_both_binary_children();
        log::trace!(
            "Array - array: {}, access: {}",
            *array.borrow(),
            *access.borrow()
        );
        let array_id = (*array.borrow()).symbol_id();
        let ret_type = (*array.borrow()).return_type().to_base_type();
        let access_with_offset = self.calc_array_index(ret_type, access);
        let array_access_retval = IOperand::from_symbol(self.make_temp(ret_type), ret_type);
        self.icode.append_statement(IStatement {
            op_type: ret_type.into(),
            operator: IOperator::Array,
            operand1: Some(IOperand::from_symbol(array_id, ret_type)),
            operand2: Some(access_with_offset),
            ret_target: Some(array_access_retval.clone()),
        });
        array_access_retval
    }

    fn calc_array_index(&mut self, base_type: ReturnType, access: SyntaxNodeBox) -> IOperand {
        let type_size: usize = IOperatorSize::from(base_type).into();
        let acccess_exp = self.accept_expression(access);
        let access_with_offset = IOperand::from_symbol(self.make_temp(base_type), base_type);
        self.icode.append_statement(IStatement {
            op_type: IOperatorSize::Double,
            operator: IOperator::Mul,
            operand1: Some(IOperand::from(type_size)),
            operand2: Some(acccess_exp),
            ret_target: Some(access_with_offset.clone()),
        });
        access_with_offset
    }

    fn make_temp(&mut self, ret_type: ReturnType) -> SymbolId {
        self.table.add_tempvar(ret_type, self.current_func())
    }

    fn make_label(&mut self) -> SymbolId {
        self.table.add_label(self.current_func())
    }
}
