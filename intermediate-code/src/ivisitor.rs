use crate::intermediate_code::IntermediateCode;
use crate::ioperand::IOperand;
use crate::ioperator::{IOperator, IOperatorType};
use crate::istatement::IStatement;
use syntax::{NodeType::*, ReturnType};
use syntax::{SymbolId, SyntaxNode};

pub struct IVisitor<'a> {
    table: &'a mut syntax::SymbolTable,
    icode: IntermediateCode,
    func_stack: Vec<SymbolId>,
    label_counter: usize,
    temp_counter: usize,
}

impl<'a> IVisitor<'a> {
    pub fn new(table: &'a mut syntax::SymbolTable) -> Self {
        Self {
            table,
            icode: IntermediateCode::new(),
            func_stack: vec![],
            label_counter: 0,
            temp_counter: 0,
        }
    }
    pub fn result(self) -> IntermediateCode {
        self.icode
    }

    fn accept(&mut self, node: &SyntaxNode) {
        log::trace!("{}", node);
        let ntype = node.node_type();
        if ntype.is_expression() {
            self.accept_expression(node);
            return;
        }
        match ntype {
            Empty => {}
            StatementList => {
                let (l, r) = node.get_binary_children();
                self.accept(l.expect("Left of StatementList was None"));
                if let Some(r) = r {
                    self.accept(r);
                }
            }
            If => {
                let (cond, targets) = node.get_both_binary_children();

                // if without else
                let (if_body, else_body) = if targets.node_type() == StatementList {
                    (targets, None)
                }
                // if with else
                else {
                    let (if_body, else_body) = targets.get_both_binary_children();
                    (if_body, Some(else_body))
                };
                self.visit_if_else(cond, if_body, else_body);
            }
            While => {
                let (cond, body) = node.get_both_binary_children();
                self.visit_while(cond, body);
            }
            Return => {
                let ret_child = node.get_unary_child();
                self.visit_return(ret_child);
            }
            _ => {
                unimplemented!("{:?}", node.node_type());
            }
        }
    }

    // TODO:
    // implement short-circuiting, i.e.
    // int x = 0 && func();
    // should never call `func`
    fn accept_expression(&mut self, exp: &SyntaxNode) -> IOperand {
        let ntype = exp.node_type();
        log::trace!("Expression: {}", ntype);
        match ntype {
            Assignment => {
                let (l, r) = exp.get_both_binary_children();
                self.visit_assignment(l, r)
            }
            Add | Sub | Mul | Div | And | Or | RelEqual | RelNotEqual | RelGT | RelGTE | RelLT
            | RelLTE => {
                let operator = IOperator::from(exp.node_type());
                let (l, r) = exp.get_both_binary_children();
                let common_ret = l.return_type();
                assert!(
                    common_ret == r.return_type(),
                    "Return types were not the same - coercion violation"
                );
                let l_expr = self.accept_expression(l);
                let r_expr = self.accept_expression(r);

                let ret = self.make_temp(common_ret);
                let ret_target = IOperand::Symbol {
                    id: ret,
                    ret_type: common_ret,
                };
                self.icode.append_statement(IStatement {
                    op_type: common_ret.into(),
                    operator,
                    operand1: Some(l_expr),
                    operand2: Some(r_expr),
                    ret_target: Some(ret_target.clone()),
                });
                ret_target
            }
            Coercion => {
                let child = exp.get_unary_child().unwrap();
                let ret_type = exp.return_type();
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
                let (func, args) = exp.get_both_binary_children();
                self.visit_func_call(func, args)
            }
            RArray => self.visit_rarray_access(exp),
            Num => {
                let num = exp.get_number();
                IOperand::Immediate {
                    value: num,
                    ret_type: exp.return_type(),
                }
            }
            Id => {
                if let SyntaxNode::Symbol {
                    symbol_id,
                    return_type,
                    ..
                } = exp
                {
                    IOperand::Symbol {
                        id: *symbol_id,
                        ret_type: *return_type,
                    }
                } else {
                    unreachable!()
                }
            }
            Not => {
                let child = exp.get_unary_child().unwrap();
                let child_exp = self.accept_expression(child);
                let ret_target = self.make_temp(ReturnType::Bool);
                let ret_target = IOperand::Symbol {
                    id: ret_target,
                    ret_type: ReturnType::Bool,
                };
                self.icode.append_statement(IStatement {
                    op_type: IOperatorType::Byte,
                    operator: IOperator::Not,
                    operand1: Some(child_exp),
                    operand2: None,
                    ret_target: Some(ret_target.clone()),
                });
                ret_target
            }
            SignMinus => {
                let child = exp.get_unary_child().unwrap();
                let ret_type = child.return_type();
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
                let child = exp.get_unary_child().unwrap();
                self.accept_expression(child)
            }
            _ => unreachable!("Node `{}` is not (part of) an expression", exp),
        }
    }

    pub fn visit_function(&mut self, func: &SyntaxNode, func_id: SymbolId) {
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
            op_type: IOperatorType::Void,
            operator: IOperator::Func,
            operand1: Some(label_istmt),
            operand2: None,
            ret_target: None,
        });
        self.accept(func);
    }

    fn current_func(&self) -> SymbolId {
        *self.func_stack.last().unwrap()
    }

    fn visit_assignment(&mut self, l_var: &SyntaxNode, r_expr: &SyntaxNode) -> IOperand {
        log::trace!("Assignment: {} | {}", l_var, r_expr);
        let common_ret = l_var.return_type().to_base_type();
        let ret_target = if l_var.return_type().is_array() {
            self.visit_larray_access(l_var)
        } else {
            IOperand::Symbol {
                id: l_var.symbol_id(),
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
        cond: &SyntaxNode,
        if_branch: &SyntaxNode,
        else_branch: Option<&SyntaxNode>,
    ) {
        let cond_expr = self.accept_expression(cond);
        let else_label = self.make_label();
        // Check condition, jump to else-label if condition was false
        self.icode.append_statement(IStatement {
            op_type: IOperatorType::Void,
            operator: IOperator::Jz,
            operand1: Some(cond_expr),
            operand2: None,
            ret_target: Some(IOperand::Symbol {
                id: else_label,
                ret_type: ReturnType::Void,
            }),
        });
        // If-body
        self.accept(if_branch);
        // else-label
        self.icode
            .append_statement(IStatement::make_label(else_label));
        if let Some(else_branch) = else_branch {
            let end_else_label = self.make_label();
            // jump over the else-body if condition was true
            self.icode.insert_statement(
                IStatement::make_goto(end_else_label),
                self.icode.n_statements() - 1,
            );
            // else-body
            self.accept(else_branch);
            // end-else label
            self.icode
                .append_statement(IStatement::make_label(end_else_label));
        }
    }

    fn visit_while(&mut self, cond: &SyntaxNode, body: &SyntaxNode) {
        let start_loop = self.make_label();
        let end_loop = self.make_label();
        // start label
        self.icode
            .append_statement(IStatement::make_label(start_loop));
        // eval expression
        let cond_expr = self.accept_expression(cond);
        // jump over while body if expression is false
        self.icode.append_statement(IStatement {
            op_type: IOperatorType::Void,
            operator: IOperator::Jz,
            operand1: Some(cond_expr),
            operand2: None,
            ret_target: Some(IOperand::Symbol {
                id: end_loop,
                ret_type: ReturnType::Void,
            }),
        });
        // while-body
        self.accept(body);
        // jump to beginning of loop
        self.icode
            .append_statement(IStatement::make_goto(start_loop));
        // end label
        self.icode
            .append_statement(IStatement::make_label(end_loop));
    }

    fn visit_func_call(&mut self, func: &SyntaxNode, args: &SyntaxNode) -> IOperand {
        let func_id = func.symbol_id();
        let ret_type = self.table.get_symbol(&func_id).unwrap().return_type;
        let ret_temp = self.make_temp(ret_type);
        let ret_temp = IOperand::Symbol {
            id: ret_temp,
            ret_type,
        };

        self.visit_expr_list(args);
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

    fn visit_expr_list(&mut self, expr_list: &SyntaxNode) {
        let (current_exp_node, next) = expr_list.get_binary_children();
        if let Some(exp_node) = current_exp_node {
            let exp = self.accept_expression(exp_node);
            self.icode.append_statement(IStatement {
                op_type: exp_node.return_type().into(),
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

    fn visit_return(&mut self, ret: Option<&SyntaxNode>) {
        let ret_exp = ret.map(|r| self.accept_expression(r));
        self.icode.append_statement(IStatement {
            op_type: IOperatorType::Void,
            operator: IOperator::Return,
            operand1: ret_exp,
            operand2: None,
            ret_target: None,
        });
    }

    fn visit_larray_access(&mut self, node: &SyntaxNode) -> IOperand {
        let (array, access) = node.get_both_binary_children();
        log::trace!("LArray - array: {}, access: {}", array, access);
        let array_id = array.symbol_id();
        let ret_type = array.return_type().to_base_type();
        let write_val = self
            .icode
            .get_statement(-2)
            .ret_target
            .as_ref()
            .expect(
                "Could not get return value of second-to-last statement as value to put into array",
            )
            .clone();
        let access_with_offset = self.calc_array_index(ret_type, access);
        let ret_target = IOperand::from_symbol(array_id, ret_type);
        self.icode.append_statement(IStatement {
            op_type: ret_type.into(),
            operator: IOperator::Larray,
            operand1: Some(write_val),
            operand2: Some(access_with_offset),
            ret_target: Some(ret_target.clone()),
        });
        ret_target
    }

    fn visit_rarray_access(&mut self, node: &SyntaxNode) -> IOperand {
        let (array, access) = node.get_both_binary_children();
        let array_id = array.symbol_id();
        let ret_type = array.return_type().to_base_type();
        let access_with_offset = self.calc_array_index(ret_type, access);
        let array_access_retval = IOperand::from_symbol(self.make_temp(ret_type), ret_type);
        self.icode.append_statement(IStatement {
            op_type: ret_type.into(),
            operator: IOperator::Rarray,
            operand1: Some(IOperand::from_symbol(array_id, ret_type)),
            operand2: Some(access_with_offset),
            ret_target: Some(array_access_retval.clone()),
        });
        array_access_retval
    }

    fn calc_array_index(&mut self, base_type: ReturnType, access: &SyntaxNode) -> IOperand {
        let type_size: usize = IOperatorType::from(base_type).into();
        let acccess_exp = self.accept_expression(access);
        let access_with_offset = IOperand::from_symbol(self.make_temp(base_type), base_type);
        self.icode.append_statement(IStatement {
            op_type: IOperatorType::Double,
            operator: IOperator::Mul,
            operand1: Some(IOperand::from(type_size)),
            operand2: Some(acccess_exp),
            ret_target: Some(access_with_offset.clone()),
        });
        access_with_offset
    }

    fn make_temp(&mut self, ret_type: ReturnType) -> SymbolId {
        let name = "&".to_string() + &self.temp_counter.to_string();
        self.temp_counter += 1;
        self.table.add_tempvar(ret_type, name, self.current_func())
    }

    fn make_label(&mut self) -> SymbolId {
        let name = "@".to_string() + &self.label_counter.to_string();
        self.label_counter += 1;
        self.table.add_label(name, self.current_func())
    }
}
