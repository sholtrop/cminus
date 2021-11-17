use std::collections::hash_map::Entry;

use lexical::{ParseTree, Rule};

use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    node::{Node, NodeType},
    scope::{Scope, ScopeManager},
    symbol::{ReturnType, Symbol, SymbolType},
    symbol_table::SymbolTable,
    syntax_tree::{FunctionRoot, SyntaxTree},
};

pub struct SyntaxBuilder {
    table: SymbolTable,
    tree: SyntaxTree,
    scope_manager: ScopeManager,
    current_function: SymbolId,
    current_line: usize,
}

impl SyntaxBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            tree: SyntaxTree::new(),
            scope_manager: ScopeManager::new(),
            current_function: SymbolId(0),
            current_line: 1,
        }
    }

    pub fn parsetree_to_syntaxtree(
        mut self,
        tree: ParseTree,
    ) -> Result<SyntaxTree, SyntaxBuilderError> {
        self.build_tree_recursive(tree)?;
        Ok(self.tree)
    }

    fn build_tree_recursive(&mut self, parse_tree: ParseTree) -> Result<(), SyntaxBuilderError> {
        for node in parse_tree {
            match node.as_rule() {
                Rule::fn_declaration => {
                    self.handle_fn_declaration(node.into_inner())?;
                }
                Rule::statement => {
                    log::trace!("statement `{}`", node.as_str());
                }

                Rule::COMMENT | Rule::WHITESPACE | Rule::EOI => {}
                Rule::linebreak => {
                    self.current_line += 1;
                }
                _ => {
                    log::warn!(
                        "Unimplemented rule `{:?}`: {}",
                        node.as_rule(),
                        node.as_str()
                    );
                }
            }
        }
        Ok(())
    }

    fn add_builtins() {
        todo!("implement")
    }

    fn add_function(&mut self, symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        if self.scope_manager.symbol_is_defined(&symbol.name) {
            return Err(format!("Symbol {} redefined in current scope", symbol.name)
                .as_str()
                .into());
        }
        let id = self.table.add_function(symbol.clone(), vec![], vec![]);
        self.scope_manager.add_symbol(id, symbol.name)?;
        Ok(id)
    }

    /// Create and enter a new function.
    fn enter_function(
        &mut self,
        name: SymbolName,
        return_type: ReturnType,
    ) -> Result<SymbolId, SyntaxBuilderError> {
        let id = self.add_function(Symbol {
            name: name.clone(),
            return_type: return_type.clone(),
            symbol_type: SymbolType::Function,
            line: self.current_line,
        })?;
        self.current_function = id;
        self.tree.functions.insert(
            id,
            FunctionRoot {
                name,
                root: Some(Node::Unary {
                    child: None,
                    node_type: NodeType::StatementList,
                    return_type,
                }),
            },
        );
        self.scope_manager.enter_new_scope();
        Ok(id)
    }

    fn leave_function(&mut self) {
        self.scope_manager.leave_scope();
    }

    fn get_id(&self, name: &str) -> Option<SymbolId> {
        todo!("Implement")
    }

    fn get_symbol_by_id(id: SymbolId) -> Symbol {
        todo!("Implement")
    }

    fn get_symbol_by_name(name: &SymbolName) -> Symbol {
        todo!("Implement")
    }

    fn get_current_function(&self) -> SymbolId {
        self.current_function.clone()
    }

    fn attach_root(name: SymbolName, new_root: Node) {
        todo!("Implement")
    }

    fn get_parameters(&self, name: &str) -> Option<Vec<Symbol>> {
        let id = self.get_id(name)?;
        self.table.get_func_param_symbols(id)
    }

    fn handle_fn_declaration(
        &mut self,
        mut func_components: ParseTree,
    ) -> Result<(), SyntaxBuilderError> {
        let type_specifier = func_components
            .next()
            .ok_or("Missing function return type specifier")?
            .as_str();
        let ident = func_components
            .next()
            .ok_or("Missing function identifier")?
            .as_str();
        let params = func_components.next().ok_or("Missing function params")?;
        let func_body = func_components.next().ok_or("Missing function body")?;
        log::trace!(
            "Function decl:\nret: {}\nname: {}\nparams: {}\nbody:\n{}",
            type_specifier,
            ident,
            params.as_str(),
            func_body.as_str(),
        );
        self.enter_function(ident.into(), type_specifier.into())?;
        self.build_tree_recursive(func_body.into_inner())?;
        self.leave_function();
        Ok(())
    }
}
