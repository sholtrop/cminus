use lexical::{ParseNode, ParseTree};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
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

pub struct SyntaxResult {
    pub tree: SyntaxTree,
    pub symbol_table: SymbolTable,
}

pub struct Visitor {
    builder: SyntaxBuilder,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            builder: SyntaxBuilder::new(),
        }
    }

    pub fn result(self) -> SyntaxResult {
        self.builder.result()
    }

    fn add_builtins(&mut self) {}

    pub fn program_start(&mut self) {
        self.builder.add_builtins();
    }

    pub fn visit_func_start(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        self.builder.enter_function(name, return_type)
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

    pub fn visit_func_end(&mut self, id: &SymbolId, root: SyntaxNode) {
        self.builder.attach_root(id, root);
        self.builder.leave_function();
    }

    pub fn add_local_scope(&mut self) {
        self.builder.enter_new_scope()
    }

    pub fn leave_local_scope(&mut self) {
        self.builder.leave_scope()
    }

    pub fn visit_func_call(
        &mut self,
        name: SymbolName,
        args: Vec<SyntaxNode>,
    ) -> Result<SyntaxNode, SyntaxBuilderError> {
        // let func_id = self.builder.get_symbol_id(name);
        // let node = SyntaxNode::Binary {
        //     node_type: NodeType::FunctionCall,
        //     return_type: ReturnType::
        // }
        todo!("func_call: Weave collected expression SyntaxNodes together in a expression list")
    }

    pub fn visit_number(&mut self, number: String) -> Result<SyntaxNode, SyntaxBuilderError> {
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
    pub fn visit_statement_list(
        &mut self,
        mut list: Vec<SyntaxNode>,
    ) -> Result<SyntaxNode, SyntaxBuilderError> {
        let mut iter = list.drain(..).rev();
        let first = iter
            .next()
            .ok_or_else(|| SyntaxBuilderError::from("Expected at least one item in `list`"))?;
        let mut stmt_list = SyntaxNode::Binary {
            left: Some(Box::new(first)),
            right: None,
            node_type: NodeType::StatementList,
            return_type: ReturnType::Void,
        };
        for node in iter {
            stmt_list = SyntaxNode::Binary {
                left: Some(Box::new(node)),
                right: Some(Box::new(stmt_list)),
                node_type: NodeType::StatementList,
                return_type: ReturnType::Void,
            }
        }
        Ok(stmt_list)
    }
}
