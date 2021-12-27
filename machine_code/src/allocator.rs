use std::{collections::HashMap, io::Write};

use intermediate_code::ioperator::IOperatorType;
use syntax::{SymbolId, SymbolTable};

use crate::register::Register;

pub type OutStream = Box<dyn Write>;

pub struct GlobalsAllocator<'a> {
    out: OutStream,
    globals: HashMap<SymbolId, IOperatorType>,
    table: &'a SymbolTable,
}

impl<'a> GlobalsAllocator<'a> {
    pub fn new(out: OutStream, table: &'a SymbolTable) -> Self {
        Self {
            out: Box::new(out),
            globals: HashMap::new(),
            table,
        }
    }

    pub fn contains(&self, id: SymbolId) -> bool {
        todo!()
    }

    pub fn insert(&self, id: SymbolId, optype: IOperatorType) {
        todo!()
    }

    pub fn load(&self, id: SymbolId, dest: &Register) {
        todo!()
    }

    pub fn load_array(&self, id: SymbolId, dest: &Register) {
        todo!()
    }

    pub fn store(&self, src: &Register, id: SymbolId) {}

    pub fn generate_data_segment(&self) {
        if self.globals.is_empty() {
            return;
        }
        todo!()
    }
}
