use intermediate_code::{ic_info::ICLineNumber, ioperand::IOperand, ioperator::IOperatorSize};
use syntax::{SymbolId, SymbolTable};

use crate::{
    allocator::GlobalsAllocator,
    assembly::asm::*,
    output::{self, OutStream},
    register::{Register, RegisterName},
};

pub struct CodeEmitter<'a> {
    out: OutStream,
    globals: &'a GlobalsAllocator<'a>,
    table: &'a SymbolTable,
    line: ICLineNumber,
}

impl<'a> CodeEmitter<'a> {
    pub fn new(out: OutStream, globals: &'a GlobalsAllocator<'a>, table: &'a SymbolTable) -> Self {
        CodeEmitter {
            out,
            globals,
            table,
            line: ICLineNumber(1),
        }
    }

    pub fn emit_store(&self, dst: &SymbolId, src: &IOperand) {
        let size = src.ret_type().into();
        let op = Op::Mov(size);
        let src = match *src {
            IOperand::Immediate { value, .. } => Src::Immediate(value),
            IOperand::Symbol { id, .. } => {
                let sym = self.table.get_symbol(&id).unwrap();
                Src::Register(Register::new(RegisterName::Rax, size))
            }
            _ => unreachable!(),
        };
        let dst = Dest::Register(Register::new(RegisterName::Rbx, size));
        self.write(Instr(op, src, dst));
    }

    fn write(&self, contents: impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
