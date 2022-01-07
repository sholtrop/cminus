use intermediate_code::{
    ic_info::ICLineNumber,
    ioperand::IOperand,
    ioperator::IOperatorSize::{self, *},
};
use syntax::{SymbolId, SymbolTable, SymbolType};

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
        self.write(&instr(op, src, dst));
    }

    pub fn emit_label(&self, id: &SymbolId) {
        let label = match self.table.get_symbol(id) {
            Some(sym) => match sym.symbol_type {
                SymbolType::Function | SymbolType::Label => Label::new(sym.name.clone()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        self.write(&label);
    }

    pub fn emit_func(&self, id: &SymbolId) {
        let name = self.table.get_symbol(id).unwrap().name.clone().0;
        self.write(&Directive::Global(name.clone()));
        self.write(&format!("{}:\n", name));
        self.emit_prologue();
    }

    /// Emits a function prologue
    pub fn emit_prologue(&self) {
        let rbp = reg(Rbp, Quad);
        let rsp = reg(Rsp, Quad);
        let push_base_p = instr(Op::Push(Quad), rbp, Dest::None);
        let save_stack_p = instr(Op::Mov(Quad), rsp, rbp);
        self.write(&push_base_p);
        self.write(&save_stack_p);
    }

    // Emits a function epilogue
    pub fn emit_epilogue(&self) {
        let rbp = reg(Rbp, Quad);
        let pop_rbp = instr(Op::Pop(Quad), rbp, Dest::None);
        self.write(&pop_rbp);
        self.write(&instr(Op::Ret, Src::None, Dest::None));
    }

    pub fn emit_return(&self, src: Src) {
        let size = match src {
            Src::Immediate(v) => Some(v.into()),
            Src::Register(r) => Some(r.optype),
            Src::Global(g) => todo!("Globals for return"),
            Src::None => None,
            _ => unreachable!("Invalid source for return: {}", src),
        };
        if let Some(size) = size {
            let rax = reg(Rax, size);
            let instr = instr(Op::Mov(size), src, rax);
            self.write(&instr);
        }
        self.emit_epilogue();
    }

    pub fn emit_call(&self, id: &SymbolId, ret: &Option<IOperand>) {
        // TODO: save parameters, then pop them in reverse order for the called function
        let func_name = self.table.get_symbol(id).unwrap().name.clone();
        self.write(&instr(Op::Call, Src::Label(func_name.0), Dest::None));
        // TODO: return value is in %rax and may need to be moved somewhere else
    }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
