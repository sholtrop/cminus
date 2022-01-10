use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

use intermediate_code::flow_graph::FlowGraph;
use intermediate_code::{ic_info::ICLineNumber, ioperator::IOperatorSize};
use syntax::{SymbolId, SymbolTable};

use crate::assembly::asm::{Directive, Label};
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
}

impl From<StoredLocation> for Src {
    fn from(loc: StoredLocation) -> Self {
        match loc {
            StoredLocation::Global(l) => Src::Global(l),
            StoredLocation::Reg(r) => Src::Register(r),
            StoredLocation::Stack(s) => Src::Stack(s),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StackOffset(pub i32);

pub struct RegAlloc<'a> {
    out: OutStream,
    reg_locals: HashMap<SymbolId, RegisterName>,
    stack_locals: HashMap<SymbolId, StackOffset>,
    param_regs: BinaryHeap<RegisterName>,
    gpurpose_regs: BinaryHeap<RegisterName>,
    table: &'a SymbolTable,
    graph: &'a FlowGraph,
    current_line: ICLineNumber,
    globals: HashMap<SymbolId, (String, IOperatorSize)>,
}

#[derive(PartialEq)]
pub enum RW {
    Read,
    Write,
}

impl<'a> RegAlloc<'a> {
    pub fn new(out: OutStream, table: &'a SymbolTable, graph: &'a FlowGraph) -> Self {
        Self {
            out,
            reg_locals: HashMap::new(),
            stack_locals: HashMap::new(),
            param_regs: BinaryHeap::from_iter([R9, R8, Rcx, Rdx, Rsi, Rdi]),
            gpurpose_regs: BinaryHeap::from_iter([R15, R14, R13, R12, R11, R10]),
            table,
            graph,
            current_line: ICLineNumber(1),
            globals: HashMap::new(),
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

    pub fn alloc_single(&mut self, id: &SymbolId, read_or_write: RW) -> StoredLocation {
        let sym = self.table.get_symbol(id).unwrap();
        let size = IOperatorSize::from(sym.return_type);
        let is_global = self.globals.contains_key(id);

        let ret = if sym.is_param() {
            // Params only go into the specified param registers
            match self.param_regs.pop() {
                Some(reg) => StoredLocation::Reg(Register::new(reg, size)),
                None => {
                    todo!("Stack offset for params")
                    // StoredLocation::Stack(StackOffset(0))
                }
            }
        } else if is_global {
            if read_or_write == RW::Read {
                // Global is already loaded into a register. Read from that register instead of .data section.
                if let Some((_, alias)) = self.reg_locals.iter().find(|(k, _)| *k == id) {
                    return StoredLocation::Reg(reg(*alias, size));
                }
            }
            let global_var = self.globals.get(id).unwrap().0.clone();
            StoredLocation::Global(global_var)
        } else {
            match self.get_register(id) {
                Some(r) => StoredLocation::Reg(reg(r, size)),
                None => {
                    todo!("Stack offset for locals")
                    // StoredLocation::Stack(0)
                }
            }
        };
        log::trace!(
            "{} will be {} into {:#?}",
            self.table.get_symbol(id).unwrap().name.0,
            if read_or_write == RW::Read {
                "read"
            } else {
                "written"
            },
            ret
        );

        ret
    }

    /// Assigns a register to a single [SymbolId] `sym` based on the following rules:
    /// 1. If `sym` is in a register R, return R and do not allocate any other register
    /// 2. If an empty register is available, return that
    /// 3. Reclaim an occupied register from a dead variable, use that
    /// 4. Return None (should allocate on stack instead)
    fn get_register(&mut self, sym: &SymbolId) -> Option<RegisterName> {
        self.get_allocated_symbol_reg(sym)
            .or_else(|| self.assign_empty_reg(*sym))
            .or_else(|| self.reclaim_dead_reg(*sym))
    }

    fn assign_empty_reg(&mut self, sym: SymbolId) -> Option<RegisterName> {
        if let Some(reg) = self.gpurpose_regs.pop() {
            self.reg_locals.insert(sym, reg);
            Some(reg)
        } else {
            None
        }
    }

    fn get_allocated_symbol_reg(&self, sym: &SymbolId) -> Option<RegisterName> {
        self.reg_locals.get(sym).cloned()
    }

    fn reclaim_dead_reg(&mut self, sym: SymbolId) -> Option<RegisterName> {
        let live_out = self.graph.get_live_out_at(&self.current_line);
        let dead_reg = self
            .reg_locals
            .iter()
            .find(|(id, _)| !live_out.contains(id))
            .map(|(_, r)| *r);
        if let Some(r) = dead_reg {
            self.reg_locals.insert(sym, r);
            Some(r)
        } else {
            None
        }
    }

    fn write(&self, contents: &impl ToString) {
        output::write(self.out.clone(), contents);
    }
}
