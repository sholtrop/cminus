use intermediate_code::{
    ic_info::ICLineNumber,
    ioperand::IOperand,
    ioperator::IOperatorSize::{self, *},
};
use syntax::{ConstantNodeValue, ReturnType, SymbolId, SymbolTable, SymbolType};

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

pub enum SignChange {
    SignedToUnsigned,
    UnsignedToSigned,
    SignedToSigned,
    UnsignedToUnsigned,
}

pub enum CastType {
    Upcast(SignChange),
    Reinterpret,
    Downcast,
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

    // pub fn emit_store(&self, dst: &SymbolId, src: &IOperand) {
    //     todo!("Register allocation");
    //     let size = src.ret_type().into();
    //     let op = Op::Mov(size);
    //     let src = match *src {
    //         IOperand::Immediate { value, .. } => Src::Immediate(value),
    //         IOperand::Symbol { id, .. } => {
    //             let sym = self.table.get_symbol(&id).unwrap();
    //             Src::Register(reg(Rax, size))
    //         }
    //         _ => unreachable!(),
    //     };
    //     let dst = Dest::Register(reg(Rbx, size));
    //     self.write(&instr(op, src, dst));
    // }

    // pub fn emit_load(&mut self, dst: &SymbolId, src: &IOperand) {}

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
        let (src, ret_type) = if let Some(retval) = retval {
            self.get_source(retval)
        } else {
            (Src::None, ReturnType::Void)
        };
        let size = ret_type.into();
        if size != IOperatorSize::Void {
            let rax = reg(Rax, size);
            let instr = instr(Op::Mov(size), src, rax);
            self.write(&instr);
        }
        // Every function has an implicit return and will therefore have an epilogue
        self.emit_epilogue();
    }

    pub fn emit_call(&self, id: &SymbolId, ret: &Option<IOperand>) {
        // TODO: save parameters, then pop them in reverse order for the called function
        let func_name = self.table.get_symbol(id).unwrap().name.clone().0;
        self.write(&instr(Op::Call, Src::Label(func_name), Dest::None));
        // TODO: return value is in %rax and may need to be moved somewhere else
    }

    /// Emits one of the following casts:
    /// ```
    /// From      To
    /// UINT      BOOL
    /// INT       UINT, BOOL
    /// UINT8     INT, UINT, BOOL
    /// INT8      UINT8, INT, UINT, BOOL
    /// BOOL      INT8, UINT8, INT, UINT
    /// ```
    pub fn emit_cast(&mut self, src: &IOperand, dest: &SymbolId) {
        let is_immediate = matches!(src, IOperand::Immediate { .. });
        let (src, src_type) = self.get_source(src);
        let dest_type = self.table.get_symbol(dest).unwrap().return_type;
        let src_size = src_type.into();
        let dest_size: IOperatorSize = dest_type.into();
        let dest = self.reg_alloc.alloc_single(dest, RW::Write);

        if is_immediate {
            let instr = instr(Op::Mov(dest_size), src, &dest);
            return self.write(&instr);
        }

        match Self::get_cast_type(src_type, dest_type) {
            CastType::Downcast => {
                // Downcast can only be x -> boolean
                let zero = match src_type {
                    ReturnType::Int8 => ConstantNodeValue::Int8(0),
                    ReturnType::Int => ConstantNodeValue::Int(0),
                    ReturnType::Uint8 => ConstantNodeValue::Uint8(0),
                    ReturnType::Uint => ConstantNodeValue::Uint(0),
                    _ => unreachable!(),
                };
                let instr_cmp = instr(Op::Comp(src_size), src, Dest::Immediate(zero));
                let instr_setne = instr(Op::Setne, Src::None, &dest);
                self.write(&instr_cmp);
                self.write(&instr_setne);
            }
            CastType::Reinterpret => {
                log::trace!("Reinterpret cast");
                // todo!("Issue mov instr if src is an immediate. Otherwise, shouldn't need to do anything");
                if let Src::Immediate(_) = src {
                    let instr = instr(Op::Mov(src_size), src, &dest);
                    self.write(&instr);
                }
            }
            CastType::Upcast(sign_change) => {
                log::trace!("Upcast");
                let instr = match sign_change {
                    SignChange::UnsignedToUnsigned | SignChange::UnsignedToSigned => {
                        instr(Op::Movz(src_size, dest_size), src, &dest)
                    }
                    SignChange::SignedToSigned | SignChange::SignedToUnsigned => {
                        instr(Op::Movs(src_size, dest_size), src, &dest)
                    }
                };
                self.write(&instr);
            }
        }
    }

    pub fn emit_add(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId) {
        let (lhs, ret_type) = self.get_source(lhs);
        let size = ret_type.into();
        let (rhs, _) = self.get_source(rhs);
        let ret = self.reg_alloc.alloc_single(ret, RW::Write);

        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                // Two immediates do not need a temp
                let instr = instr(Op::Mov(size), Src::Immediate(x + y), &ret);
                self.write(&instr);
            }
        } else {
            let instr1 = instr(Op::Mov(size), lhs, &ret);
            let instr2 = instr(Op::Add(size), rhs, &ret);
            self.write(&instr1);
            self.write(&instr2);
        };
    }

    pub fn emit_assign(&mut self, src: &IOperand, dest: &SymbolId) {
        let (src, src_ret) = self.get_source(src);
        let dest = self.reg_alloc.alloc_single(dest, RW::Write);
        let instr = instr(Op::Mov(src_ret.into()), src, &dest);
        self.write(&instr);
    }

    pub fn emit_sub(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId) {
        let (lhs, ret_type) = self.get_source(lhs);
        let size = ret_type.into();
        let (rhs, _) = self.get_source(rhs);
        let ret = self.reg_alloc.alloc_single(ret, RW::Write);

        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                // Two immediates do not need a temp
                let instr = instr(Op::Mov(size), Src::Immediate(x - y), &ret);
                self.write(&instr);
            }
        } else {
            let instr1 = instr(Op::Mov(size), lhs, &ret);
            let instr2 = instr(Op::Sub(size), rhs, &ret);
            self.write(&instr1);
            self.write(&instr2);
        };
    }

    fn get_source(&mut self, src: &IOperand) -> (Src, ReturnType) {
        match *src {
            IOperand::Immediate { value, ret_type } => (Src::Immediate(value), ret_type),
            IOperand::Symbol { id, ret_type } => {
                let r = self.reg_alloc.alloc_single(&id, RW::Read);
                (r.into(), ret_type)
            }
            IOperand::Unknown => unreachable!(),
        }
    }

    fn get_cast_type(from: ReturnType, to: ReturnType) -> CastType {
        let from_size: IOperatorSize = from.into();
        let to_size = &to.into();
        log::trace!("{} {}", from_size, to_size);
        match from_size.cmp(to_size) {
            std::cmp::Ordering::Equal => CastType::Reinterpret,
            std::cmp::Ordering::Less => {
                let sign_change = match (from.is_unsigned(), to.is_unsigned()) {
                    (true, true) => SignChange::UnsignedToUnsigned,
                    (false, false) => SignChange::SignedToSigned,
                    (true, false) => SignChange::UnsignedToSigned,
                    (false, true) => SignChange::SignedToUnsigned,
                };
                CastType::Upcast(sign_change)
            }
            std::cmp::Ordering::Greater => CastType::Downcast,
        }
    }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
