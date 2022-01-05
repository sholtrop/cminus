use intermediate_code::{ic_info::ICLineNumber, ioperand::IOperand, ioperator::IOperatorSize::*};
use syntax::{SymbolId, SymbolTable};

use crate::{
    assembly::asm::*,
    global_alloc::GlobalsAllocator,
    output::{self, OutStream},
    register::{reg, RegisterName::*},
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
        todo!("Register allocation");
        let size = src.ret_type().into();
        let op = Op::Mov(size);
        let src = match *src {
            IOperand::Immediate { value, .. } => Src::Immediate(value),
            IOperand::Symbol { id, .. } => {
                let sym = self.table.get_symbol(&id).unwrap();
                Src::Register(reg(Rax, size))
            }
            _ => unreachable!(),
        };
        let dst = Dest::Register(reg(Rbx, size));
        self.write(instr(op, src, dst));
    }

    /// Emits a function prologue
    pub fn emit_prologue(&self) {
        let rbp = reg(Rbp, Quad);
        let rsp = reg(Rsp, Quad);
        let push_base_p = instr(Op::Push(Quad), rbp, Dest::None);
        let save_stack_p = instr(Op::Mov(Quad), rsp, rbp);
        self.write(push_base_p);
        self.write(save_stack_p);
    }

    // Emits a function epilogue
    pub fn emit_epilogue(&self) {
        let rbp = reg(Rbp, Quad);
        let pop_rbp = instr(Op::Pop(Quad), rbp, Dest::None);
        self.write(pop_rbp);
        self.write(Op::Ret);
    }

    fn write(&self, contents: impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
