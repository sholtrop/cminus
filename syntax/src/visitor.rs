use std::collections::HashMap;

use lexical::{ParseNode, ParseTree};

use crate::{
    builder::SyntaxBuilder,
    error::SyntaxBuilderError,
    id::SymbolName,
    symbol::{ReturnType, Symbol},
    symbol_table::SymbolTable,
    syntax_tree::SyntaxTree,
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
        // self.builder.scope_manager.
    }

    pub fn visit_func_start(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
    ) -> Result<(), SyntaxBuilderError> {
        self.builder.enter_function(name, return_type).and(Ok(()))
    }

    // pub fn visit_parameters(&mut self) {

    //     // let variables = variables
    //     //     .into_iter()
    //     //     .map(|var| self.add_symbol(var, scope))
    //     //     .collect();
    //     let parameters = parameters
    //         .into_iter()
    //         .map(|param| self.add_symbol(param, scope))
    //         .collect();
    // }

    pub fn visit_func_end(&mut self) {
        self.builder.leave_function();
    }
}
