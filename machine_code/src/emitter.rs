use intermediate_code::{
    ic_info::ICLineNumber,
    ioperand::IOperand,
    ioperator::{
        IOperator,
        IOperatorSize::{self, *},
    },
};
use syntax::{ConstantNodeValue, ReturnType, SymbolId, SymbolName, SymbolTable, SymbolType};

use crate::{
    assembly::asm::*,
    output::{self, OutStream},
    reg_alloc::{AllocType, RegAlloc, StoredLocation},
    register::{reg, RegisterName::*},
};

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
pub struct CodeEmitter<'a> {
    out: OutStream,
    reg_alloc: RegAlloc<'a>,
    table: &'a SymbolTable,
    line: ICLineNumber,
    current_func: Option<SymbolId>,
}

impl<'a> CodeEmitter<'a> {
    pub fn new(out: OutStream, reg_alloc: RegAlloc<'a>, table: &'a SymbolTable) -> Self {
        CodeEmitter {
            out,
            reg_alloc,
            table,
            line: ICLineNumber(1),
            current_func: None,
        }
    }

    pub fn set_line(&mut self, line: ICLineNumber) {
        self.line = line;
        self.reg_alloc.set_line(line);
    }

    pub fn emit_global_decls(&mut self) {
        self.reg_alloc.generate_data_segment();
    }

    pub fn emit_goto(&mut self, label: &SymbolId) {
        let label_name = self.get_label_name(label);
        let jump_instr = instr(Op::Jmp, Src::Label(label_name.to_string()), Dest::None);
        self.write(&jump_instr);
    }

    pub fn emit_conditional_jump(
        &mut self,
        jump_type: &IOperator,
        label: &SymbolId,
        l_exp: &IOperand,
        r_exp: Option<&IOperand>,
    ) {
        let op = match *jump_type {
            IOperator::Je | IOperator::Jz => Op::Je,
            IOperator::Jne | IOperator::Jnz => Op::Jne,
            IOperator::Ja => Op::Ja,
            IOperator::Jae => Op::Jae,
            IOperator::Jb => Op::Jb,
            IOperator::Jbe => Op::Jbe,
            IOperator::Jg => Op::Jg,
            IOperator::Jge => Op::Jge,
            IOperator::Jl => Op::Jl,
            IOperator::Jle => Op::Jle,
            _ => unreachable!(),
        };
        log::trace!("JUMP {} {:?}", l_exp, r_exp);
        let (l, ret) = self.get_source(l_exp);
        let r = if let Some(r_exp) = r_exp {
            self.get_source(r_exp).0
        } else {
            assert_eq!(op, Op::Jne);
            Src::Immediate(ConstantNodeValue::from(0))
        };
        if let Src::Immediate(l) = l {
            if let Src::Immediate(r) = r {
                // Fold two constants because a cmp instruction cannot operate on two constants
                let holds = match op {
                    Op::Je => l == r,
                    Op::Jne => l != r,
                    Op::Ja | Op::Jg => l > r,
                    Op::Jae | Op::Jge => l >= r,
                    Op::Jb | Op::Jl => l < r,
                    Op::Jbe | Op::Jle => l <= r,
                    _ => unreachable!(),
                };
                if holds {
                    self.emit_goto(label);
                }
                return;
            }
        }
        // AT&T syntax means l and r are reversed for cmp
        let comp_instr = instr2(Op::Comp(ret.into()), r, l);
        self.write(&comp_instr);
        let label_name = self.get_label_name(label);
        let jump_instr = instr(op, Src::Label(label_name.to_string()), Dest::None);
        self.write(&jump_instr);
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

    pub fn emit_func(&mut self, id: &SymbolId) {
        self.current_func = Some(*id);
        let name = self.table.get_symbol(id).unwrap().name.clone().0;
        self.write(&Directive::Global(name.clone()));
        self.write(&format!("{}:\n", name));
        self.emit_prologue();
        let params = self.table.get_func_param_ids(id).unwrap();
        let locals_size = self.reg_alloc.alloc_stack_locals(id);
        if locals_size.0 > 0 {
            let instr = instr(
                Op::Sub(Quad),
                Src::Immediate(ConstantNodeValue::from(locals_size.0 as i64)),
                reg(Rsp, Quad),
            );
            self.write(&instr);
        }
        self.reg_alloc.alloc_func_params(params);
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
        let leave = instr(Op::Leave, Src::None, Dest::None);
        self.write(&leave);
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

    pub fn emit_param(&mut self, value: &IOperand) {
        let size = value.ret_type().into();
        let dest: Dest = (&self.reg_alloc.alloc_call_param(size)).into();
        let src = match value {
            IOperand::Immediate { value, .. } => Src::Immediate(*value),
            IOperand::Symbol { id, .. } => {
                let loc = self.reg_alloc.alloc_var(id, AllocType::Read);
                loc.into()
            }
            _ => panic!("Unknown operand value {}", value),
        };
        let instr = instr(Op::Mov(size), src, dest);
        self.write(&instr);
    }

    pub fn emit_call(&mut self, id: &SymbolId, ret: &Option<SymbolId>) {
        let func_sym = self.table.get_symbol(id).unwrap();
        let func_name = func_sym.name.clone().0;
        let func_ret_size = func_sym.return_type.into();
        self.write(&instr(Op::Call, Src::Label(func_name), Dest::None));
        if let Some(ret) = ret {
            if func_ret_size != IOperatorSize::Void {
                let size = self.table.get_symbol(ret).unwrap().return_type.into();
                let dest = self.reg_alloc.alloc_var(ret, AllocType::Write);
                let rax = reg(Rax, func_ret_size);
                let instr = instr(Op::Mov(size), rax, &dest);
                self.write(&instr);
            }
        }
        self.reg_alloc.free_param_regs();
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
        let dest = self.reg_alloc.alloc_var(dest, AllocType::Write);

        if is_immediate {
            let instr = instr(Op::Mov(dest_size), src, &dest);
            return self.write(&instr);
        }

        match Self::get_cast_type(src_type, dest_type) {
            CastType::Downcast => {
                // Downcast can only be x -> boolean
                let zero = ConstantNodeValue::from(0);
                let instr_cmp = instr(Op::Comp(src_size), src, Dest::Immediate(zero));
                // let setne_src: Src = dest.into();
                // let instr_setne = instr(Op::Setne, setne_src, Dest::None);
                self.write(&instr_cmp);
                // self.write(&instr_setne);
            }
            CastType::Reinterpret => {
                log::trace!("Reinterpret cast");
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
        let ret = self.reg_alloc.alloc_var(ret, AllocType::Write);

        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                // Constant-fold two immediates
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
        log::trace!("{} {}", src, src_ret);
        let dest = self.reg_alloc.alloc_var(dest, AllocType::Write);
        let instr = instr(Op::Mov(src_ret.into()), src, &dest);
        self.write(&instr);
    }

    pub fn emit_sub(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId) {
        let (lhs, ret_type) = self.get_source(lhs);
        let size = ret_type.into();
        let (rhs, _) = self.get_source(rhs);
        let ret = self.reg_alloc.alloc_var(ret, AllocType::Write);

        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                // Constant-fold two immediates
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

    pub fn emit_mul(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId) {
        let is_unsigned = lhs.ret_type().is_unsigned();
        let (lhs, ret_type) = self.get_source(lhs);
        let size = ret_type.into();
        let (rhs, _) = self.get_source(rhs);
        let ret = self.reg_alloc.alloc_var(ret, AllocType::Write);
        if let Src::Immediate(x) = lhs {
            if let Src::Immediate(y) = rhs {
                // Constant-fold two immediates
                let instr = instr(Op::Mov(size), Src::Immediate(x * y), &ret);
                self.write(&instr);
            }
        } else {
            let op = if is_unsigned {
                Op::Mul(size)
            } else {
                Op::IMul(size)
            };
            let instr1 = instr(Op::Mov(size), lhs, &ret);
            let instr2 = instr(op, rhs, &ret);
            self.write(&instr1);
            self.write(&instr2);
        };
    }

    pub fn emit_div_mod(&mut self, lhs: &IOperand, rhs: &IOperand, ret: &SymbolId, op: &IOperator) {
        let (lhs, _) = self.get_source(lhs);
        // divisor must be in a register
        let (rhs, _) = self.get_reg_source(rhs);

        let mov_instr = instr(Op::Mov(Double), lhs, reg(Rax, Double));
        self.write(&mov_instr);
        let edx = reg(Rdx, Double);
        let xor_instr = instr(Op::Xor(Double), edx, edx);
        self.write(&xor_instr);
        let div_instr = instr(Op::Div, &rhs, Dest::None);
        self.write(&div_instr);
        let ret_size = self.table.get_symbol(ret).unwrap().return_type.into();
        let dest = self.reg_alloc.alloc_var(ret, AllocType::Write);
        let ret_val = if *op == IOperator::Mod {
            reg(Rdx, ret_size)
        } else if *op == IOperator::Div {
            reg(Rax, ret_size)
        } else {
            unreachable!()
        };
        let mov_to_ret = instr(Op::Mov(ret_size), ret_val, &dest);
        self.write(&mov_to_ret);

        if let StoredLocation::TempReg(_, id) = rhs {
            self.reg_alloc.free_temp(&id);
        }
    }

    pub fn emit_set(
        &mut self,
        set_type: &IOperator,
        lhs: &IOperand,
        rhs: &IOperand,
        dest: &SymbolId,
    ) {
        let (l, ret) = self.get_source(lhs);
        let (r, _) = self.get_source(rhs);
        let size = ret.into();
        let dest = self.reg_alloc.alloc_var(dest, AllocType::Write);
        let op = match *set_type {
            IOperator::SetE => Op::SetE,
            IOperator::SetNE => Op::SetNE,
            IOperator::SetA => Op::SetA,
            IOperator::SetAE => Op::SetAE,
            IOperator::SetB => Op::SetB,
            IOperator::SetBE => Op::SetBE,
            IOperator::SetG => Op::SetG,
            IOperator::SetGE => Op::SetGE,
            IOperator::SetL => Op::SetL,
            IOperator::SetLE => Op::SetLE,
            _ => unreachable!(),
        };
        log::trace!("COMP; {} {}", l, r);
        let cmp_instr = instr2(Op::Comp(size), r, l);
        self.write(&cmp_instr);
        let set_instr = instr(op, &dest, Dest::None);
        self.write(&set_instr);
    }

    fn get_source(&mut self, src: &IOperand) -> (Src, ReturnType) {
        match *src {
            IOperand::Immediate { value, ret_type } => (Src::Immediate(value), ret_type),
            IOperand::Symbol { id, ret_type } => {
                let r = self.reg_alloc.alloc_var(&id, AllocType::Read);
                (r.into(), ret_type)
            }
            IOperand::Unknown => unreachable!(),
        }
    }

    fn get_reg_source(&mut self, src: &IOperand) -> (StoredLocation, ReturnType) {
        match *src {
            IOperand::Immediate { ret_type, .. } => {
                let (id, regname) = self.reg_alloc.alloc_temp();
                let reg = reg(regname, ret_type.into());
                (StoredLocation::TempReg(reg, id), ret_type)
            }
            IOperand::Symbol { id, ret_type } => {
                let reg = self.reg_alloc.alloc_var(&id, AllocType::ReadReg);
                (reg, ret_type)
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

    fn get_label_name(&self, label: &SymbolId) -> &SymbolName {
        match self.table.get_symbol(label) {
            Some(sym) => match sym.symbol_type {
                SymbolType::Label => &sym.name,
                _ => unreachable!("Not a label symbol"),
            },
            _ => unreachable!("Label not found in symbol table"),
        }
    }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
