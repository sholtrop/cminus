use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

use intermediate_code::flow_graph::FlowGraph;
use intermediate_code::{ic_info::ICLineNumber, ioperator::IOperatorSize};
use syntax::{SymbolId, SymbolTable};

use crate::assembly::asm::{instr, Directive, Label, Op};
use crate::register::reg;
use crate::{assembly::asm::Src, output};
use crate::{
    output::OutStream,
    register::{Register, RegisterName, RegisterName::*},
};

#[derive(Clone, Debug)]
pub enum StoredLocation {
    Reg(Register),
    Stack(StackOffset),
    Global(String),
    TempReg(Register, AllocId),
}

impl From<StoredLocation> for Src {
    fn from(loc: StoredLocation) -> Self {
        match loc {
            StoredLocation::Global(l) => Src::Global(l),
            StoredLocation::Reg(r) | StoredLocation::TempReg(r, _) => Src::Register(r),
            StoredLocation::Stack(s) => Src::Stack(s),
        }
    }
}

#[derive(PartialEq)]
pub enum AllocType {
    Read,
    Write,
    ReadReg, // var must be read from a register
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AllocId {
    Sym(SymbolId),
    Temp(usize),
}

impl AllocId {
    pub fn to_id(&self) -> Option<SymbolId> {
        if let Self::Sym(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}

impl From<SymbolId> for AllocId {
    fn from(id: SymbolId) -> Self {
        Self::Sym(id)
    }
}

const N_PARAM_REGS: usize = 6;
const PARAM_REGS: [RegisterName; N_PARAM_REGS] = [R9, R8, Rcx, Rdx, Rsi, Rdi];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StackOffset(pub usize);

impl std::fmt::Display for StackOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0 => "".into(),
                _ => format!("-{}", self.0),
            }
        )
    }
}

const STACK_ALIGN: usize = 16;
pub struct RegAlloc<'a> {
    out: OutStream,
    reg_locals: HashMap<AllocId, RegisterName>,
    stack_locals: HashMap<AllocId, StackOffset>,
    param_regs: Vec<RegisterName>,
    gpurpose_regs: BinaryHeap<RegisterName>,
    table: &'a SymbolTable,
    graph: &'a FlowGraph,
    current_line: ICLineNumber,
    globals: HashMap<SymbolId, (String, IOperatorSize)>,
    temp_counter: usize,
}

impl<'a> RegAlloc<'a> {
    pub fn new(out: OutStream, table: &'a SymbolTable, graph: &'a FlowGraph) -> Self {
        Self {
            out,
            reg_locals: HashMap::new(),
            stack_locals: HashMap::new(),
            param_regs: Vec::from_iter(PARAM_REGS),
            gpurpose_regs: BinaryHeap::from_iter([
                R15, R14, R13, R12, /* R11, R10 are caller-saved and get clobbered */
            ]),
            table,
            graph,
            current_line: ICLineNumber(1),
            globals: HashMap::new(),
            temp_counter: 0,
        }
    }

    pub fn generate_data_segment(&mut self) {
        let globals = self.table.get_globals();
        if globals.is_empty() {
            return;
        }
        self.write(&Label::new(".LCX"));
        for (id, sym) in globals {
            let size = sym.return_type.into();
            self.globals.insert(id, (sym.name.to_string(), size));
            if sym.is_array() {
                todo!("Implement array globals")
            } else {
                self.write(&Directive::Comm {
                    name: sym.name.to_string(),
                    size,
                });
            }
        }
    }

    pub fn set_line(&mut self, line: ICLineNumber) {
        self.current_line = line;
    }

    pub fn alloc_var(&mut self, id: &SymbolId, alloc_type: AllocType) -> StoredLocation {
        let aid = AllocId::from(*id);
        let sym = self.table.get_symbol(id).unwrap();
        let size = IOperatorSize::from(sym.return_type);
        let is_global = self.globals.contains_key(id);

        let stored_location = if is_global {
            match alloc_type {
                AllocType::ReadReg => {
                    // if let Some((_, alias)) =
                    //     self.get_non_temp_regs().iter().find(|(k, _)| *k == *id)
                    // {
                    //     StoredLocation::Reg(reg(**alias, size))
                    // } else {
                    let regname = self
                        .alloc_register(&aid)
                        .expect("No registers free for global alloc");
                    let (var, size) = self.globals.get(id).unwrap();
                    let register = reg(regname, *size);
                    let mov_instr = instr(Op::Mov(*size), Src::Global(var.clone()), register);
                    self.write(&mov_instr);
                    StoredLocation::Reg(register)
                    // }
                }
                _ => {
                    let global_var = self.globals.get(id).unwrap().0.clone();
                    StoredLocation::Global(global_var)
                }
            }
        } else {
            match alloc_type {
                AllocType::ReadReg => {
                    let reg = if let Some(regname) = self.reg_locals.get(&aid) {
                        reg(*regname, size)
                    } else {
                        let regname = self.alloc_register(&aid).expect("No free regs");
                        let size = self.table.get_symbol(id).unwrap().return_type.into();
                        let offset = self.get_stack_local(&aid).expect("Var not on stack");
                        let register = reg(regname, size);
                        let mov_instr = instr(Op::Mov(size), Src::Stack(offset), register);
                        self.write(&mov_instr);
                        register
                    };
                    StoredLocation::Reg(reg)
                }
                _ => {
                    if let Some(regname) = self.reg_locals.get(&aid) {
                        StoredLocation::Reg(reg(*regname, size))
                    } else if let Some(offset) = self.get_stack_local(&aid) {
                        StoredLocation::Stack(offset)
                    } else {
                        let regname = self.alloc_register(&aid).expect("No free regs");
                        StoredLocation::Reg(reg(regname, size))
                    }
                }
            }
        };
        stored_location
    }

    pub fn alloc_func_params(&mut self, params: &[SymbolId]) {
        if params.len() > N_PARAM_REGS {
            todo!("Allocate stack space for param passing");
        }
        for (idx, param) in params.iter().enumerate() {
            let reg = PARAM_REGS[N_PARAM_REGS - idx - 1];
            self.reg_locals.insert((*param).into(), reg);
            log::trace!("Alloc {} to {:#?}", param, reg);
        }
    }

    pub fn alloc_call_param(&mut self, size: IOperatorSize) -> StoredLocation {
        // Params only go into the specified param registers
        match self.param_regs.pop() {
            Some(reg) => StoredLocation::Reg(Register::new(reg, size)),
            None => {
                todo!("Stack offset for params");
            }
        }
    }

    pub fn free_param_regs(&mut self) {
        log::debug!("Freeing param regs");
        self.param_regs = Vec::from_iter(PARAM_REGS);
    }

    pub fn alloc_stack_locals(&mut self, func: &SymbolId) -> StackOffset {
        let func = self.table.get_func_var_symbols(func).unwrap();
        let mut top_of_stack = 0_usize;
        for (id, sym) in func {
            let size: IOperatorSize = sym.return_type.into();
            let size = usize::from(size);
            let offset = top_of_stack + size;
            self.stack_locals.insert(id.into(), StackOffset(offset));
            top_of_stack += size;
        }
        let padding = (STACK_ALIGN - (top_of_stack % STACK_ALIGN)) % STACK_ALIGN;
        StackOffset(top_of_stack + padding)
    }

    pub fn get_stack_local(&mut self, sym: &AllocId) -> Option<StackOffset> {
        log::trace!("Getting {:?}", sym);
        log::trace!("{:#?}", self.stack_locals);
        self.stack_locals.get(sym).copied()
    }

    pub fn alloc_temp(&mut self) -> (AllocId, RegisterName) {
        let aid = AllocId::Temp(self.temp_counter);
        self.temp_counter += 1;
        let reg = self.alloc_register(&aid).expect("Could not get register");
        (aid, reg)
    }

    pub fn free_temp(&mut self, id: &AllocId) {
        let regname = self
            .reg_locals
            .remove(id)
            .unwrap_or_else(|| panic!("No such temp allocated: {:#?}", id));
        self.gpurpose_regs.push(regname);
    }

    /// Assigns a register to a single [SymbolId] `sym` based on the following rules:
    /// 1. If `sym` is in a register R, return R and do not allocate any other register
    /// 2. If an empty register is available, return that
    /// 3. Reclaim an occupied register from a dead variable, use that
    /// 4. Return None (should allocate on stack instead)
    fn alloc_register(&mut self, id: &AllocId) -> Option<RegisterName> {
        self.assign_empty_reg(*id)
            .or_else(|| self.reclaim_dead_reg(*id))
        // .or_else(|| self.reclaim_temp_reg(*id))
    }

    fn assign_empty_reg(&mut self, id: AllocId) -> Option<RegisterName> {
        if let Some(reg) = self.gpurpose_regs.pop() {
            self.reg_locals.insert(id, reg);
            log::debug!("Allocating reg {:?}", reg);
            Some(reg)
        } else {
            None
        }
    }

    fn reclaim_dead_reg(&mut self, aid: AllocId) -> Option<RegisterName> {
        let live_out = self.graph.get_live_out_at(&self.current_line);
        let live_in = self.graph.get_live_at(&self.current_line);
        log::debug!("Liveout: {:#?}\nLive: {:#?}", live_out, live_in);
        if let Some(r) = self
            .get_non_temp_regs()
            .iter()
            .find(|(id, _)| !live_out.contains(id))
            .map(|(_, r)| **r)
        {
            self.reg_locals.insert(aid, r);
            log::debug!("Overwriting reg {:?}", r);
            Some(r)
        } else {
            None
        }
    }

    // fn reclaim_temp_reg(&mut self, aid: AllocId) -> Option<RegisterName> {
    //     if let Some((_, temp)) = self.get_temp_regs().first() {
    //         let temp = **temp;
    //         self.reg_locals.insert(aid, temp);
    //         Some(temp)
    //     } else {
    //         None
    //     }
    // }

    fn get_non_temp_regs(&self) -> Vec<(SymbolId, &RegisterName)> {
        let vec: Vec<_> = self
            .reg_locals
            .iter()
            .filter_map(|(k, r)| k.to_id().map(|k| (k, r)))
            .collect();
        vec
    }

    // fn get_temp_regs(&self) -> Vec<(AllocId, &RegisterName)> {
    //     let vec: Vec<_> = self
    //         .reg_locals
    //         .iter()
    //         .filter_map(|(k, v)| match k {
    //             AllocId::Temp(_) => Some((*k, v)),
    //             _ => None,
    //         })
    //         .collect();
    //     vec
    // }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
