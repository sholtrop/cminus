use syntax::SymbolTable;

use crate::allocator::OutStream;

pub struct CodeGenerator {
    out: OutStream,
}

impl CodeGenerator {
    pub fn new(out: OutStream) -> Self {
        Self { out }
    }

    pub fn generate_header(&mut self) {
        self.out
            .write_all(b"# Output generated by the CoCo Compiler\n")
            .unwrap();
    }

    pub fn generate_global_decls(&mut self, table: &SymbolTable) {
        for sym in table.get_globals() {}
    }
}
