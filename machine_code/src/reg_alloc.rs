use std::{
    collections::{BTreeSet, HashMap},
    iter::FromIterator,
};

use intermediate_code::istatement::IStatement;
use syntax::SymbolId;

use crate::{
    output::OutStream,
    register::{Register, RegisterName, RegisterName::*},
};

pub struct RegisterAllocator {
    out: OutStream,
    regs: HashMap<SymbolId, RegisterName>,
    param_regs: BTreeSet<RegisterName>,
    other_regs: BTreeSet<RegisterName>,
}

impl RegisterAllocator {
    pub fn new(out: OutStream) -> Self {
        Self {
            out,
            regs: HashMap::new(),
            param_regs: BTreeSet::from_iter([Rdi, Rsi, Rdx, Rcx, R8, R9]),
            other_regs: BTreeSet::from_iter([R10, R11, R12, R13, R14, R15]),
        }
    }

    pub fn get_register(op: &IStatement) -> [Option<Register>; 3] {
        [None, None, None]
    }
}
