use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;

use crate::error::SyntaxBuilderWarning;
use crate::node::{ConstantNodeValue, NodeType};
use crate::syntax_tree::SyntaxTree;
use crate::{
    builder::SyntaxBuilder,
    error::SyntaxBuilderError,
    id::BUILTIN_IDS,
    id::{SymbolId, SymbolName},
    node::SyntaxNode,
    symbol::{ReturnType, Symbol, SymbolType},
    symbol_table::{SymbolTable, SYMBOL_ID_ERROR},
};

pub struct SyntaxAnalysisResult {
    pub tree: SyntaxTree,
    pub symbol_table: SymbolTable,
    pub errors: Vec<ErrorWithLineno>,
    pub warnings: Vec<WarningWithLineno>,
}

type ErrorWithLineno = (SyntaxBuilderError, usize);
type WarningWithLineno = (SyntaxBuilderWarning, usize);

#[derive(Default)]
pub struct Visitor {
    builder: SyntaxBuilder,
    errors: Vec<ErrorWithLineno>,
    warnings: Vec<WarningWithLineno>,
    current_line: usize,
}

pub type SyntaxResult = Result<SyntaxNode, SyntaxBuilderError>;

impl Visitor {
    pub fn new() -> Self {
        Self {
            builder: SyntaxBuilder::new(),
            errors: vec![],
            warnings: vec![],
            current_line: 1,
        }
    }

    pub fn result(self) -> SyntaxAnalysisResult {
        let (symbol_table, tree) = self.builder.result();
        SyntaxAnalysisResult {
            symbol_table,
            tree,
            errors: self.errors,
            warnings: self.warnings,
        }
    }

    fn add_builtins(&mut self) {
        let old_line = self.current_line;
        self.current_line = 0;

        // writeinteger
        let id = self
            .visit_func_start(
                SymbolType::Function,
                ReturnType::Void,
                SymbolName::from("writeinteger"),
            )
            .expect("Error adding builtins: Function `writeinteger` start");
        assert_ne!(
            self.visit_param_decl(SymbolName::from("i"), ReturnType::Int)
                .0,
            SYMBOL_ID_ERROR,
            "Error adding builtins: Param `i` for `writeinteger`"
        );

        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `writeinteger` end");
        // writeunsigned
        let id = self
            .visit_func_start(
                SymbolType::Function,
                ReturnType::Void,
                SymbolName::from("writeunsigned"),
            )
            .expect("Error adding builtins: Function `writeunsigned` start");
        assert_eq!(id.0, BUILTIN_IDS[1]);
        assert_ne!(
            self.visit_param_decl(SymbolName::from("i"), ReturnType::Uint)
                .0,
            SYMBOL_ID_ERROR,
            "Error adding builtins: Param `i` for `writeunsigned`"
        );
        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `writeunsigned` end");
        // readinteger
        let id = self
            .visit_func_start(
                SymbolType::Function,
                ReturnType::Int,
                SymbolName::from("readinteger"),
            )
            .expect("Error adding builtins: Function `readinteger` start");
        assert_eq!(id.0, BUILTIN_IDS[2]);
        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `readinteger` end");
        let id = self
            .visit_func_start(
                SymbolType::Function,
                ReturnType::Int,
                SymbolName::from("readunsigned"),
            )
            .expect("Error adding builtins: Function `readunsigned` start");
        assert_eq!(id.0, BUILTIN_IDS[3]);
        self.visit_func_end(&id, SyntaxNode::Empty)
            .expect("Error adding builtins: Function `readunsigned` end");
        self.current_line = old_line;
    }

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    pub fn increase_line_nr(&mut self, increase: usize) {
        self.current_line += increase;
    }

    pub fn program_start(&mut self) {
        self.add_builtins();
    }

    /// Register a function and return its [SymbolId],
    /// or an error if it is already defined.
    pub fn visit_func_start(
        &mut self,
        symbol_type: SymbolType,
        return_type: ReturnType,
        name: SymbolName,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        let id = self
            .builder
            .enter_function(Symbol {
                name,
                return_type,
                symbol_type,
                line: self.current_line,
            })
            .map_err(|e| {
                self.add_error(&e);
                e
            })?;
        self.add_local_scope();
        Ok(id)
    }

    pub fn visit_param_decl(&mut self, name: SymbolName, return_type: ReturnType) -> SymbolId {
        self.builder
            .add_symbol(Symbol {
                name,
                return_type,
                symbol_type: SymbolType::Parameter,
                line: self.current_line,
            })
            .unwrap_or_else(|err| {
                self.add_error(&err);
                SymbolId(SYMBOL_ID_ERROR)
            })
    }

    /// Declare a new variable and return its [SymbolId].
    /// Returns an error if the variable has already been declared in this scope.
    /// TODO: Mentions parameter shadowing in error if applicable
    pub fn visit_var_decl(&mut self, name: SymbolName, return_type: ReturnType) -> SymbolId {
        self.builder
            .add_symbol(Symbol {
                name,
                return_type,
                symbol_type: SymbolType::Variable,
                line: self.current_line,
            })
            .unwrap_or_else(|err| {
                self.add_error(&err);
                SymbolId(SYMBOL_ID_ERROR)
            })
    }

    pub fn visit_func_end(
        &mut self,
        id: &SymbolId,
        root: SyntaxNode,
    ) -> Result<(), SyntaxBuilderError> {
        self.builder.attach_root(id, root)?;
        self.builder.leave_function();
        self.leave_local_scope();
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
        actual_args: Vec<SyntaxNode>,
    ) -> SyntaxNode {
        let (func, id) = match self.builder.get_symbol_by_name(name) {
            Some((f, s)) => (f, s),
            None => {
                let err = SyntaxBuilderError(format!("Cannot find function with name `{}`", name));
                self.add_error(&err);
                return err.into();
            }
        };
        // Leave me alone, mr. borrow checker
        let func = func.clone();
        let mut current_node: Option<SyntaxNode> = None;

        if let SymbolType::Function = func.symbol_type {
            let formal_args = match self.builder.get_parameters(&id) {
                Ok(v) => v,
                Err(e) => {
                    self.add_error(&e);
                    return e.into();
                }
            };
            let formal_args = formal_args.into_iter();
            let actual_args = actual_args.into_iter();
            let n_formal_args = formal_args.len();
            let n_actual_args = actual_args.len();
            match n_actual_args.cmp(&n_formal_args) {
                Ordering::Greater => {
                    let err = SyntaxBuilderError(format!(
                        "Too many arguments for function {}. Expected {}, got {}",
                        func.name, n_formal_args, n_actual_args
                    ));
                    self.add_error(&err);
                    return err.into();
                }
                Ordering::Less => {
                    let err = SyntaxBuilderError(format!(
                        "Too few arguments for function {}. Expected {}, got {}",
                        func.name, n_formal_args, n_actual_args
                    ));
                    self.add_error(&err);
                    return err.into();
                }
                _ => {}
            };

            // TODO: Just use `zip`
            for pair in actual_args.zip_longest(formal_args).rev() {
                if let EitherOrBoth::Both(mut actual_arg, formal_arg) = pair {
                    actual_arg = SyntaxNode::coerce(actual_arg, formal_arg.return_type)
                        .unwrap_or_else(|err| {
                            self.add_error(&err);
                            err.into()
                        });
                    current_node = Some(SyntaxNode::Binary {
                        node_type: NodeType::ExpressionList,
                        return_type: ReturnType::Void,
                        left: Some(Box::new(actual_arg)),
                        right: current_node.map(Box::new),
                    });
                }
            }
        } else {
            let err = SyntaxBuilderError(format!("Symbol `{}` is not a function", name));
            self.add_error(&err);
            return err.into();
        };
        SyntaxNode::Binary {
            left: Some(Box::new(SyntaxNode::Symbol {
                node_type: NodeType::Id,
                return_type: ReturnType::Void,
                symbol_id: id,
            })),
            right: current_node.map(Box::new),
            node_type: NodeType::FunctionCall,
            return_type: func.return_type,
        }
    }

    pub fn visit_number(&mut self, number: String) -> SyntaxNode {
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
            let err =
                SyntaxBuilderError(format!("Could not convert {} to any number type", number));
            self.add_error(&err);
            return err.into();
        };
        node
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
            let current_func = self
                .builder
                .get_current_function()
                .expect("Error: No current function set");
            let current_ret = current_func.return_type;
            if current_ret == ReturnType::Void {
                let err = SyntaxBuilderError(format!(
                    "Void function `{}` can not return a value",
                    current_func.name
                ));
                self.add_error(&err);
                return err.into();
            }
            ret_node = SyntaxNode::coerce(ret_node, current_ret).unwrap_or_else(|err| {
                self.add_error(&err);
                err.into()
            });

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

    pub fn visit_while(&mut self, mut expression: SyntaxNode, statement: SyntaxNode) -> SyntaxNode {
        expression = match SyntaxNode::coerce(expression, ReturnType::Bool) {
            Ok(n) => n,
            Err(err) => {
                self.add_error(&err);
                return err.into();
            }
        };
        SyntaxNode::Binary {
            node_type: NodeType::While,
            return_type: ReturnType::Void,
            left: Some(Box::new(expression)),
            right: Some(Box::new(statement)),
        }
    }

    pub fn visit_if(
        &mut self,
        mut condition: SyntaxNode,
        if_body: SyntaxNode,
        else_body: Option<SyntaxNode>,
    ) -> SyntaxNode {
        condition = match SyntaxNode::coerce(condition, ReturnType::Bool) {
            Ok(n) => n,
            Err(err) => {
                self.add_error(&err);
                return err.into();
            }
        };

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
        SyntaxNode::Binary {
            node_type: NodeType::If,
            return_type: ReturnType::Void,
            left: Some(Box::new(condition)),
            right: Some(Box::new(rchild)),
        }
    }

    pub fn visit_assignment(&mut self, lvar: SyntaxNode, mut exp: SyntaxNode) -> SyntaxNode {
        let ret_type = lvar.return_type();
        exp = match SyntaxNode::coerce(exp, ret_type) {
            Ok(n) => n,
            Err(err) => {
                self.add_error(&err);
                return err.into();
            }
        };

        SyntaxNode::Binary {
            node_type: NodeType::Assignment,
            return_type: ret_type,
            left: Some(Box::new(lvar)),
            right: Some(Box::new(exp)),
        }
    }

    pub fn visit_unary(&mut self, mut op: SyntaxNode, unary_child: SyntaxNode) -> SyntaxNode {
        let op_type = op.node_type();
        if let SyntaxNode::Unary {
            ref mut child,
            ref mut return_type,
            ..
        } = op
        {
            if op_type == NodeType::Not {
                *return_type = ReturnType::Bool;
                let unary_child =
                    SyntaxNode::coerce(unary_child, ReturnType::Bool).unwrap_or_else(|e| {
                        self.add_error(&e);
                        e.into()
                    });
                *child = Some(Box::new(unary_child))
            } else {
                *return_type = unary_child.return_type();
                *child = Some(Box::new(unary_child));
            }
        }
        op
    }

    pub fn visit_binary(
        &mut self,
        mut left_child: SyntaxNode,
        op: &mut SyntaxNode,
        mut right_child: SyntaxNode,
    ) {
        use NodeType::*;
        let common_ret_type;
        if let SyntaxNode::Binary {
            ref mut return_type,
            ref mut left,
            ref mut right,
            node_type,
        } = op
        {
            if left_child.return_type() == right_child.return_type() {
                common_ret_type = left_child.return_type();
            } else if left_child.return_type() < right_child.return_type() {
                common_ret_type = right_child.return_type();
                left_child = SyntaxNode::coerce(left_child, common_ret_type).unwrap_or_else(|e| {
                    self.add_error(&e);
                    e.into()
                });
            } else {
                common_ret_type = left_child.return_type();
                right_child =
                    SyntaxNode::coerce(right_child, common_ret_type).unwrap_or_else(|e| {
                        self.add_error(&e);
                        e.into()
                    });
            }

            *return_type = match node_type {
                RelGT | RelGTE | RelLT | RelLTE | RelNotEqual | RelEqual | And | Or => {
                    ReturnType::Bool
                }
                _ => common_ret_type,
            };
            *left = Some(Box::new(left_child));
            *right = Some(Box::new(right_child));
        } else {
            self.add_error(&SyntaxBuilderError(format!(
                "Node {} is not a binary operator",
                op
            )));
        }
    }

    pub fn visit_variable(&mut self, name: &SymbolName) -> SyntaxNode {
        let (symbol, id) = match self.builder.get_symbol_by_name(name) {
            Some((s, i)) => (s, i),
            None => {
                let err = SyntaxBuilderError(format!("Error: Symbol `{}` is not defined", name));
                self.add_error(&err);
                return err.into();
            }
        };
        SyntaxNode::Symbol {
            node_type: NodeType::Id,
            return_type: symbol.return_type,
            symbol_id: id,
        }
    }

    pub fn visit_array_access(&mut self, name: &SymbolName, expr: SyntaxNode) -> SyntaxNode {
        let (symbol, id) = match self.builder.get_symbol_by_name(name) {
            Some((s, i)) => (s, i),
            None => {
                let err = SyntaxBuilderError(format!("Symbol `{}` is not defined", name));
                self.add_error(&err);
                return err.into();
            }
        };

        if !symbol.is_array() {
            return SyntaxBuilderError(format!("Symbol {} is not an array", name)).into();
        }
        let id_node = SyntaxNode::Symbol {
            node_type: NodeType::Id,
            return_type: symbol.return_type,
            symbol_id: id,
        };
        SyntaxNode::Binary {
            node_type: NodeType::LArray,
            return_type: symbol.return_type.to_base_type(),
            left: Some(Box::new(id_node)),
            right: Some(Box::new(expr)),
        }
    }

    pub fn visit_array_decl(
        &mut self,
        name: SymbolName,
        arr_size: SyntaxNode,
        base_type: ReturnType,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        let size = match arr_size {
            SyntaxNode::Constant { value, .. } => {
                let value = i64::from(value);
                if value < 1 {
                    return Err("Array size must be greater than 0".into());
                }
                value as usize
            }
            _ => return Err("`size` was not a Constant number SyntaxNode".into()),
        };
        let arr_symbol = Symbol {
            line: self.current_line,
            name,
            return_type: base_type.to_array_type(),
            symbol_type: SymbolType::ArrayVariable { size },
        };
        let id = self.builder.add_symbol(arr_symbol).map_err(|err| {
            self.add_error(&err);
            err
        })?;
        Ok(id)
    }

    pub fn visit_array_param_decl(&mut self, name: SymbolName, base_type: ReturnType) -> SymbolId {
        let arr_symbol = Symbol {
            line: self.current_line,
            name,
            return_type: base_type.to_array_type(),
            symbol_type: SymbolType::ArrayParam,
        };
        self.builder.add_symbol(arr_symbol).unwrap_or_else(|err| {
            self.add_error(&err);
            SymbolId(SYMBOL_ID_ERROR)
        })
    }

    pub fn add_error(&mut self, err: &SyntaxBuilderError) {
        self.errors.push((err.clone(), self.current_line))
    }

    pub fn add_warning(&mut self, warning: &SyntaxBuilderWarning) {
        self.warnings.push((warning.clone(), self.current_line))
    }
}
