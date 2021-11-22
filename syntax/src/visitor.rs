use general::tree::ArenaTree;
use lexical::{ParseNode, ParseTree};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::ops::DerefMut;
use std::{collections::HashMap, rc::Rc};

use crate::node::NodeType;
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
    statement_list: Vec<SyntaxNode>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            builder: SyntaxBuilder::new(),
            statement_list: vec![],
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
    ) -> Result<(), SyntaxBuilderError> {
        self.builder.enter_function(name, return_type).and(Ok(()))
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

    pub fn visit_func_end(&mut self, name: SymbolName, root: SyntaxNode) {
        self.builder.attach_root(name, root);
        self.builder.leave_function();
    }

    pub fn add_local_scope(&mut self) {
        self.builder.enter_new_scope()
    }

    pub fn leave_local_scope(&mut self) {
        self.builder.leave_scope()
    }

    pub fn visit_statement(&mut self, node: SyntaxNode) {
        self.statement_list.push(node);
        // match stmt_list {
        //     ListTree {
        //         current: None,
        //         root: None,
        //     } => {
        //         stmt_list.root = Some(node);
        //         stmt_list.current = Some(stmt_list.root.as_ref().unwrap().borrow_mut());
        //     }
        //     ListTree {
        //         current: Some(current),
        //         root: Some(_),
        //     } => {
        //         // current.replace(Node::Binary {
        //         //     node_type: NodeType::StatementList,
        //         //     left: node,
        //         //     right:
        //         // });
        //     }
        //     ListTree {
        //         current: None,
        //         root: Some(_),
        //     } => panic!("Invariant violated: `current` is None while `root` is Some"),
        //     ListTree {
        //         current: Some(_),
        //         root: None,
        //     } => panic!("Invariant violated: `root` is None while `current` is Some"),
        // }
    }
}
