use itertools::{EitherOrBoth, Itertools};
use lexical::{ParseNode, ParseTree};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::cmp::Ordering;
use std::ops::DerefMut;
use std::{collections::HashMap, rc::Rc};

use crate::node::{ConstantNodeValue, NodeType};
use crate::syntax_tree::SyntaxTree;
use crate::{
    builder::SyntaxBuilder,
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    node::SyntaxNode,
    symbol::{ReturnType, Symbol, SymbolType},
    symbol_table::SymbolTable,
};

pub struct FullSyntaxTree {
    pub tree: SyntaxTree,
    pub symbol_table: SymbolTable,
}

pub struct Visitor {
    builder: SyntaxBuilder,
}

pub type SyntaxResult = Result<SyntaxNode, SyntaxBuilderError>;

impl Visitor {
    pub fn new() -> Self {
        Self {
            builder: SyntaxBuilder::new(),
        }
    }

    pub fn result(self) -> FullSyntaxTree {
        self.builder.result()
    }

    fn add_builtins(&mut self) {
        let id = self
            .visit_func_start(Symbol {
                name: SymbolName::from("writeinteger"),
                return_type: ReturnType::Void,
                symbol_type: SymbolType::Function,
                line: 0,
            })
            .expect("Error adding builtins: Function `writeinteger` start");
        self.add_local_scope();
        self.visit_param_decl(SymbolName::from("i"), ReturnType::Int, 0)
            .expect("Error adding builtins: Param `i` for `writeinteger`");
        self.leave_local_scope();
        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `writeinteger` end");

        let id = self
            .visit_func_start(Symbol {
                name: SymbolName::from("readinteger"),
                return_type: ReturnType::Int,
                symbol_type: SymbolType::Function,
                line: 0,
            })
            .expect("Error adding builtins: Function `readinteger` start");
        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `readinteger` end");
    }

    pub fn program_start(&mut self) {
        self.add_builtins();
    }

    /// Register a function and return its [SymbolId],
    /// or an error if it is already defined.
    pub fn visit_func_start(
        &mut self,
        func_symbol: Symbol,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        self.builder.enter_function(func_symbol)
    }

    pub fn visit_param_decl(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
        line: usize,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        self.builder.add_symbol(Symbol {
            name,
            return_type,
            symbol_type: SymbolType::Parameter,
            line,
        })
    }

    /// Declare a new variable and return its [SymbolId].
    /// Returns an error if the variable has already been declared in this scope.
    /// TODO: Mentions parameter shadowing in error if applicable
    pub fn visit_var_decl(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
        line: usize,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        self.builder.add_symbol(Symbol {
            name,
            return_type,
            symbol_type: SymbolType::Variable,
            line,
        })
    }

    pub fn visit_func_end(
        &mut self,
        id: &SymbolId,
        root: SyntaxNode,
    ) -> Result<(), SyntaxBuilderError> {
        self.builder.attach_root(id, root)?;
        self.builder.leave_function();
        Ok(())
    }

    pub fn add_local_scope(&mut self) {
        self.builder.enter_new_scope()
    }

    pub fn leave_local_scope(&mut self) {
        self.builder.leave_scope()
    }

    pub fn visit_func_call(
        &mut self,
        name: &SymbolName,
        mut actual_args: Vec<SyntaxNode>,
    ) -> SyntaxResult {
        let (func, id) = self.builder.get_symbol_by_name(name).ok_or_else(|| {
            SyntaxBuilderError::from(format!("Cannot find function with name `{}`", name))
        })?;

        let mut current_node: Option<SyntaxNode> = None;

        if let SymbolType::Function = func.symbol_type {
            let formal_args = self.builder.get_parameters(&id)?.into_iter();
            let actual_args = actual_args.into_iter();
            let n_formal_args = formal_args.len();
            let n_actual_args = actual_args.len();
            match n_actual_args.cmp(&n_formal_args) {
                Ordering::Greater => {
                    return Err(SyntaxBuilderError(format!(
                        "Too many arguments for function {}. Expected {}, got {}",
                        func.name, n_formal_args, n_actual_args
                    )))
                }
                Ordering::Less => {
                    return Err(SyntaxBuilderError(format!(
                        "Too few arguments for function {}. Expected {}, got {}",
                        func.name, n_formal_args, n_actual_args
                    )))
                }
                _ => {}
            };

            for pair in actual_args.zip_longest(formal_args).rev() {
                if let EitherOrBoth::Both(mut actual_arg, formal_arg) = pair {
                    if actual_arg.return_type() != formal_arg.return_type {
                        actual_arg = SyntaxNode::coerce(actual_arg, formal_arg.return_type)
                            .unwrap_or_else(SyntaxNode::from);
                    }

                    current_node = Some(SyntaxNode::Binary {
                        node_type: NodeType::ExpressionList,
                        return_type: ReturnType::Void,
                        left: Some(Box::new(actual_arg)),
                        right: current_node.map(Box::new),
                    });
                }
            }
        } else {
            return Err(SyntaxBuilderError(format!(
                "Symbol `{}` is not a function",
                name
            )));
        };
        let func_node = SyntaxNode::Binary {
            left: Some(Box::new(SyntaxNode::Symbol {
                node_type: NodeType::Id,
                return_type: ReturnType::Void,
                symbol_id: id,
            })),
            right: current_node.map(Box::new),
            node_type: NodeType::FunctionCall,
            return_type: func.return_type.clone(),
        };
        Ok(func_node)
    }

    pub fn visit_number(&mut self, number: String) -> SyntaxResult {
        let node = if let Ok(num) = number.parse::<i8>() {
            SyntaxNode::Constant {
                node_type: NodeType::Num,
                value: ConstantNodeValue::Int8(num),
                return_type: ReturnType::Int8,
            }
        } else if let Ok(num) = number.parse::<u8>() {
            SyntaxNode::Constant {
                node_type: NodeType::Num,
                value: ConstantNodeValue::Uint8(num),
                return_type: ReturnType::Uint8,
            }
        } else if let Ok(num) = number.parse::<i32>() {
            SyntaxNode::Constant {
                node_type: NodeType::Num,
                value: ConstantNodeValue::Int(num),
                return_type: ReturnType::Int,
            }
        } else if let Ok(num) = number.parse::<u32>() {
            SyntaxNode::Constant {
                node_type: NodeType::Num,
                value: ConstantNodeValue::Uint(num),
                return_type: ReturnType::Uint,
            }
        } else {
            return Err(SyntaxBuilderError::from(
                format!("Could not convert {} to any number type", number).as_str(),
            ));
        };
        Ok(node)
    }

    /// Take a `list` of [SyntaxNode]s and weave them together by making them the left child of a StatementList and linking the StatementLists.
    pub fn visit_statement_list(&mut self, list: Vec<SyntaxNode>) -> SyntaxNode {
        let mut stmt_list: Option<SyntaxNode> = None;

        for node in list.into_iter().rev() {
            stmt_list = Some(SyntaxNode::Binary {
                left: Some(Box::new(node)),
                right: stmt_list.map(Box::new),
                node_type: NodeType::StatementList,
                return_type: ReturnType::Void,
            });
        }
        stmt_list.unwrap_or(SyntaxNode::Empty)
    }

    pub fn visit_return(&mut self, ret_node: Option<SyntaxNode>) -> SyntaxNode {
        if let Some(mut ret_node) = ret_node {
            let current_ret = self
                .builder
                .get_current_function()
                .expect("Error: No current function set")
                .return_type;
            if ret_node.return_type() != current_ret {
                ret_node =
                    SyntaxNode::coerce(ret_node, current_ret).unwrap_or_else(SyntaxNode::from)
            }
            SyntaxNode::Unary {
                node_type: NodeType::Return,
                return_type: ret_node.return_type(),
                child: Some(Box::new(ret_node)),
            }
        } else {
            SyntaxNode::Unary {
                node_type: NodeType::Return,
                return_type: ReturnType::Void,
                child: None,
            }
        }
    }

    pub fn visit_while(
        &mut self,
        mut expression: SyntaxNode,
        statement: SyntaxNode,
    ) -> SyntaxResult {
        if expression.return_type() != ReturnType::Bool {
            expression = SyntaxNode::coerce(expression, ReturnType::Bool)?;
        }
        let while_node = SyntaxNode::Binary {
            node_type: NodeType::While,
            return_type: ReturnType::Void,
            left: Some(Box::new(expression)),
            right: Some(Box::new(statement)),
        };
        Ok(while_node)
    }

    pub fn visit_if(
        &mut self,
        mut condition: SyntaxNode,
        if_body: SyntaxNode,
        else_body: Option<SyntaxNode>,
    ) -> SyntaxResult {
        let cond_ret = condition.return_type();
        if cond_ret != ReturnType::Bool {
            condition = SyntaxNode::coerce(condition, ReturnType::Bool)?;
        }
        let rchild = if let Some(else_body) = else_body {
            SyntaxNode::Binary {
                node_type: NodeType::IfTargets,
                return_type: ReturnType::Void,
                left: Some(Box::new(if_body)),
                right: Some(Box::new(else_body)),
            }
        } else {
            if_body
        };
        Ok(SyntaxNode::Binary {
            node_type: NodeType::If,
            return_type: ReturnType::Void,
            left: Some(Box::new(condition)),
            right: Some(Box::new(rchild)),
        })
    }

    pub fn visit_assignment(&mut self, lvar: SyntaxNode, mut exp: SyntaxNode) -> SyntaxResult {
        // let symbol = self.builder.get_symbol_by_id(&lvar).ok_or_else(|| {
        //     SyntaxBuilderError(format!(
        //         "Variable with id {} does not exist in the current scope",
        //         lvar
        //     ))
        // })?;

        let ret_type = lvar.return_type();

        if exp.return_type() != ret_type {
            exp = SyntaxNode::coerce(exp, ret_type)?;
        }
        // let id_node = SyntaxNode::Symbol {
        //     node_type: NodeType::Id,
        //     return_type: ret_type,
        //     symbol_id: lvar,
        // };
        let node = SyntaxNode::Binary {
            node_type: NodeType::Assignment,
            return_type: ret_type,
            left: Some(Box::new(lvar)),
            right: Some(Box::new(exp)),
        };

        Ok(node)
    }

    pub fn visit_unary(&mut self, mut op: SyntaxNode, unary_child: SyntaxNode) -> SyntaxResult {
        let op_type = op.node_type();
        if let SyntaxNode::Unary {
            ref mut child,
            ref mut return_type,
            ..
        } = op
        {
            if op_type == NodeType::Not {
                *return_type = ReturnType::Bool;
                *child = Some(Box::new(SyntaxNode::coerce(unary_child, ReturnType::Bool)?))
            } else {
                *return_type = unary_child.return_type();
                *child = Some(Box::new(unary_child));
            }
        }
        Ok(op)
    }

    pub fn visit_binary(
        &self,
        mut left_child: SyntaxNode,
        op: &mut SyntaxNode,
        mut right_child: SyntaxNode,
    ) -> Result<(), SyntaxBuilderError> {
        use NodeType::*;
        let common_ret_type;
        if let SyntaxNode::Binary {
            ref mut return_type,
            ref mut left,
            ref mut right,
            node_type,
        } = op
        {
            match node_type {
                RelGT | RelGTE | RelLT | RelLTE | RelNotEqual | RelEqual | And | Or => {
                    left_child = SyntaxNode::coerce(left_child, ReturnType::Bool)?;
                    right_child = SyntaxNode::coerce(right_child, ReturnType::Bool)?;
                    common_ret_type = ReturnType::Bool;
                }
                _ => {
                    if left_child.return_type() == right_child.return_type() {
                        common_ret_type = left_child.return_type();
                    } else if left_child.return_type() < right_child.return_type() {
                        common_ret_type = right_child.return_type();
                        left_child = SyntaxNode::coerce(left_child, common_ret_type)?;
                    } else {
                        common_ret_type = left_child.return_type();
                        right_child = SyntaxNode::coerce(right_child, common_ret_type)?;
                    }
                }
            };

            *return_type = common_ret_type;
            *left = Some(Box::new(left_child));
            *right = Some(Box::new(right_child));
        } else {
            return Err(SyntaxBuilderError(format!(
                "Node {} is not a binary operator",
                op
            )));
        }
        Ok(())
    }

    pub fn visit_lvariable(&self, name: &SymbolName) -> SyntaxResult {
        let (symbol, id) = self.builder.get_symbol_by_name(name).ok_or_else(|| {
            SyntaxBuilderError(format!("Error: Symbol `{}` is not defined", name))
        })?;
        let node = SyntaxNode::Symbol {
            node_type: NodeType::Id,
            return_type: symbol.return_type,
            symbol_id: id,
        };
        Ok(node)
    }

    pub fn visit_larray(&self, name: &SymbolName, expr: SyntaxNode) -> SyntaxResult {
        let (symbol, id) = self
            .builder
            .get_symbol_by_name(name)
            .ok_or_else(|| SyntaxBuilderError(format!("Symbol `{}` is not defined", name)))?;
        if !symbol.is_array() {
            return Err(SyntaxBuilderError(format!(
                "Symbol {} is not an array",
                name
            )));
        }
        let id_node = SyntaxNode::Symbol {
            node_type: NodeType::Id,
            return_type: symbol.return_type,
            symbol_id: id,
        };
        let access_node = SyntaxNode::Binary {
            node_type: NodeType::LArray,
            return_type: symbol.return_type,
            left: Some(Box::new(id_node)),
            right: Some(Box::new(expr)),
        };
        Ok(access_node)
    }
}
