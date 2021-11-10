use crate::{
    error::SyntaxBuilderError,
    id::{FunctionId, SymbolId},
    node::Node,
    scope::Scope,
    symbol::Symbol,
    symbol_table::SymbolTable,
    syntax_tree::SyntaxTree,
};

pub struct SyntaxBuilder {
    table: SymbolTable,
    tree: SyntaxTree,
    scope_stack: Vec<Scope>,
    current_function: FunctionId,
}

impl SyntaxBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            tree: SyntaxTree::new(),
            scope_stack: vec![Scope::new()],
            current_function: FunctionId(0),
        }
    }

    pub fn enter_new_scope() {}

    pub fn exit_current_scope() {}

    pub fn add_symbol(symbol: Symbol) -> Result<SymbolId, SyntaxBuilderError> {
        todo!("Implement")
    }

    pub fn enter_function(function: Symbol) -> Result<FunctionId, SyntaxBuilderError> {
        todo!("Implement")
    }

    pub fn leave_function() {}

    pub fn get_id(&self, name: &str) -> SymbolId {
        todo!("Implement")
    }

    pub fn get_symbol_by_id(id: SymbolId) -> Symbol {
        todo!("Implement")
    }

    pub fn get_symbol_by_name(name: &str) -> Symbol {
        todo!("Implement")
    }

    pub fn get_current_function(&self) -> FunctionId {
        self.current_function.clone()
    }

    pub fn attach_root(name: &str, new_root: Node) {
        todo!("Implement")
    }

    pub fn get_parameters(&self, name: &str) -> Option<Vec<Symbol>> {
        self.table
            .get_func_parameter_symbols(self.get_id(name).into())

        // todo!("Implement")
    }
}
