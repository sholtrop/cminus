use intermediate_code::{
    ic_info::ICLineNumber,
    ioperand::IOperand,
    ioperator::IOperatorSize::{self, *},
    istatement::IStatement,
};
use syntax::{SymbolId, SymbolTable, SymbolType};

use crate::{
    assembly::asm::*,
    output::{self, OutStream},
    reg_alloc::{RegAlloc, RW},
    register::{reg, RegisterName::*},
};

pub struct CodeEmitter<'a> {
    out: OutStream,
    reg_alloc: RegAlloc<'a>,
    table: &'a SymbolTable,
    line: ICLineNumber,
}

impl<'a> CodeEmitter<'a> {
    pub fn new(out: OutStream, reg_alloc: RegAlloc<'a>, table: &'a SymbolTable) -> Self {
        CodeEmitter {
            out,
            reg_alloc,
            table,
            line: ICLineNumber(1),
        }
    }

    pub fn set_line(&mut self, line: ICLineNumber) {
        self.line = line;
        self.reg_alloc.set_line(line);
    }

    pub fn emit_global_decls(&mut self) {
        self.reg_alloc.generate_data_segment();
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

    pub fn emit_load(&mut self, dst: &SymbolId, src: &IOperand) {}

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

    /// Emits a function epilogue
    pub fn emit_epilogue(&self) {
        let rbp = reg(Rbp, Quad);
        let pop_rbp = instr(Op::Pop(Quad), rbp, Dest::None);
        self.write(&pop_rbp);
        self.write(&instr(Op::Ret, Src::None, Dest::None));
    }

    pub fn emit_return(&mut self, retval: Option<&IOperand>) {
        let src = match retval {
            Some(IOperand::Immediate { value, .. }) => Src::Immediate(*value),
            None => Src::None,
            Some(IOperand::Symbol { id, .. }) => self.reg_alloc.alloc_single(id, RW::Read).into(),
            _ => unreachable!(),
        };

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
        // Every function has an implicit return and will therefore have an epilogue
        self.emit_epilogue();
    }

    pub fn emit_call(&self, id: &SymbolId, ret: &Option<IOperand>) {
        // TODO: save parameters, then pop them in reverse order for the called function
        let func_name = self.table.get_symbol(id).unwrap().name.clone();
        self.write(&instr(Op::Call, Src::Label(func_name.0), Dest::None));
        // TODO: return value is in %rax and may need to be moved somewhere else
    }

    pub fn emit_cast(&self, src: &SymbolId, dest: &SymbolId) {
        let src_type = self.table.get_symbol(src).unwrap().return_type;
        let dest_type = self.table.get_symbol(dest).unwrap().return_type;

        // let instr = match dest_type {
        //     Bool => {
        //         instr(Op::Setnz, src, Dest::)
        //     }
        // }
    }

    pub fn emit_add(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId) {
        let size = lhs.ret_type().into();
        let lhs = match *lhs {
            IOperand::Immediate { value, .. } => Src::Immediate(value),
            IOperand::Symbol { id, .. } => self.reg_alloc.alloc_single(&id, RW::Read).into(),
            IOperand::Unknown => unreachable!(),
        };
        let rhs = match *rhs {
            IOperand::Immediate { value, .. } => Src::Immediate(value),
            IOperand::Symbol { id, .. } => self.reg_alloc.alloc_single(&id, RW::Read).into(),
            IOperand::Unknown => unreachable!(),
        };
        let ret = self.reg_alloc.alloc_single(ret, RW::Write);

        // Two immediates do not need a temp
        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                let instr = instr(Op::Mov(size), Src::Immediate(x + y), &ret);
                self.write(&instr);
            }
        } else {
            let instr1 = instr(Op::Add(size), lhs, &ret);
            let instr2 = instr(Op::Add(size), rhs, &ret);
            self.write(&instr1);
            self.write(&instr2);
        };
    }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
