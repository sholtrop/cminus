use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

use intermediate_code::{flow_graph::FlowGraph, istatement::IStatement};
use intermediate_code::{
    ic_info::ICLineNumber,
    ioperator::{IOperator::*, IOperatorSize},
};
use syntax::{SymbolId, SymbolTable};

use crate::{
    output::OutStream,
    register::{Register, RegisterName, RegisterName::*},
};

#[derive(PartialEq)]
pub enum StoredLocation {
    Reg(RegisterName),
    Stack { rsp_offset: i32 },
}

pub struct RegisterAllocator<'a> {
    out: OutStream,
    locals: HashMap<SymbolId, StoredLocation>,
    param_regs: BinaryHeap<RegisterName>,
    gpurpose_regs: BinaryHeap<RegisterName>,
    table: &'a SymbolTable,
    graph: &'a FlowGraph,
    current_line: ICLineNumber,
}

impl<'a> RegisterAllocator<'a> {
    pub fn new(out: OutStream, table: &'a SymbolTable, graph: &'a FlowGraph) -> Self {
        Self {
            out,
            locals: HashMap::new(),
            param_regs: BinaryHeap::from_iter([R9, R8, Rcx, Rdx, Rsi, Rdi]),
            gpurpose_regs: BinaryHeap::from_iter([R15, R14, R13, R12, R11, R10]),
            table,
            graph,
            current_line: ICLineNumber(1),
        }
    }

    pub fn set_line(&mut self, line: ICLineNumber) {
        self.current_line = line;
    }

    pub fn get_register(&mut self, line: &ICLineNumber, op: &IStatement) -> [Option<Register>; 3] {
        match op.operator {
            Param => {
                let param_id = op.operand1.as_ref().unwrap().id();
                let param_size: IOperatorSize =
                    self.table.get_symbol(&param_id).unwrap().return_type.into();

                if let Some(reg) = self.param_regs.pop() {
                    let reg = Some(Register::new(reg, param_size));
                    [reg, None, None]
                } else {
                    todo!("Stack allocation of params");
                }
            }
            _ => [None, None, None],
        }
    }

    /// Assigns a register to `sym` based on the following rules:
    /// 1. If `sym` is in a register R, return R and do not allocate any other register
    /// 2. If an empty register is available, return that
    /// 3. Reclaim a register occupied by a variable that is also in another register, use that
    /// 4. Reclaim an occupied register from a dead variable, use that
    /// 5. Return None (should allocate on stack instead)
    fn allocate_register(&mut self, sym: &SymbolId) -> Option<RegisterName> {
        if let Some(&StoredLocation::Reg(r)) = self.locals.get(sym) {
            return Some(r);
        }
        if !self.gpurpose_regs.is_empty() {
            return self.gpurpose_regs.pop();
        }
        // TODO: 3.
        let live_out = self.graph.get_live_out_at(&self.current_line);
        let dead_reg = self.locals.iter().find(|(id, loc)| {
            if !live_out.contains(id) {
                if let StoredLocation::Reg(_) = loc {
                    return true;
                }
            }
            false
        });
        if let Some((_, StoredLocation::Reg(r))) = dead_reg {
            return Some(*r);
        }
        // TODO: 5.
        None
    }
}
