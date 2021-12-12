use syntax::SyntaxNode;

use crate::intermediate_code::IntermediateCode;

pub struct IVisitor {
    table: syntax::SymbolTable,
    icode: Vec<IntermediateCode>,
}

impl IVisitor {
    pub fn new(table: syntax::SymbolTable) -> Self {
        Self {
            table,
            icode: vec![],
        }
    }

    pub fn visit_function(&mut self, func: SyntaxNode) {}
}
