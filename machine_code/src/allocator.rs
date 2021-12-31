use intermediate_code::ioperator::IOperatorSize;
use std::collections::HashMap;
use syntax::{SymbolId, SymbolTable};

use crate::assembly::asm::*;
use crate::output;
use crate::register::Register;
use crate::{output::OutStream, register::RegisterName};

pub struct GlobalsAllocator<'a> {
    out: OutStream,
    globals: HashMap<SymbolId, IOperatorSize>,
    table: &'a SymbolTable,
}

impl<'a> GlobalsAllocator<'a> {
    pub fn new(out: OutStream, table: &'a SymbolTable) -> Self {
        Self {
            out,
            globals: HashMap::new(),
            table,
        }
    }

    pub fn insert(&mut self, id: SymbolId, optype: IOperatorSize) {
        self.globals.insert(id, optype);
    }

    pub fn load(&self, id: &SymbolId, dest: &RegisterName) {
        let optype = self.table.get_symbol(id).unwrap().return_type.into();
        let reg = Register::new(*dest, optype);
        self.write(Instr(
            Op::Mov(optype),
            Src::Global(*id),
            Dest::Register(reg),
        ));
    }

    pub fn load_array(&self, id: &SymbolId, dest: &RegisterName) {
        todo!()
    }

    pub fn store(&self, src: &RegisterName, id: &SymbolId) {
        let optype = self.table.get_symbol(id).unwrap().return_type.into();
        let reg = Register::new(*src, optype);
        self.write(Instr(
            Op::Mov(optype),
            Src::Register(reg),
            Dest::Global(id.to_string()),
        ));
    }

    pub fn generate_data_segment(&mut self) {
        if self.globals.is_empty() {
            return;
        }
        self.write(Label::new("LCX"));
        for (id, optype) in &self.globals {
            let symbol = self.table.get_symbol(id).unwrap();
            if symbol.is_array() {
                todo!("Implement array globals")
            } else {
                self.write(Directive::Comm {
                    name: id.to_string(),
                    size: *optype,
                });
            }
        }
    }

    fn write(&self, contents: impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
