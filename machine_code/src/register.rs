use intermediate_code::ioperator::IOperatorSize;
use std::fmt;

pub struct Register {
    pub name: RegisterName,
    pub optype: IOperatorSize,
}

impl Register {
    pub fn new(name: RegisterName, optype: IOperatorSize) -> Self {
        Self { name, optype }
    }
}

#[derive(Clone, Copy)]
pub enum RegisterName {
    Invalid,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub enum RegisterType {
    GeneralPurpose,
    SpecialPurpose,
}

impl From<&RegisterName> for RegisterType {
    fn from(name: &RegisterName) -> Self {
        match name {
            RegisterName::Rax
            | RegisterName::Rbx
            | RegisterName::Rcx
            | RegisterName::Rdx
            | RegisterName::Rsi
            | RegisterName::Rdi
            | RegisterName::Rbp
            | RegisterName::Rsp => RegisterType::SpecialPurpose,
            _ => RegisterType::GeneralPurpose,
        }
    }
}

impl Register {
    fn get_start(&self) -> String {
        match RegisterType::from(&self.name) {
            RegisterType::GeneralPurpose => "",
            RegisterType::SpecialPurpose => match self.optype {
                IOperatorSize::Byte | IOperatorSize::Word => "",
                IOperatorSize::Double => "e",
                IOperatorSize::Quad => "r",
                _ => unreachable!(),
            },
        }
        .to_string()
    }

    fn get_middle(&self) -> String {
        match self.name {
            RegisterName::Rax => "a",
            RegisterName::Rbx => "b",
            RegisterName::Rcx => "c",
            RegisterName::Rdx => "d",
            RegisterName::Rsi => "si",
            RegisterName::Rdi => "di",
            RegisterName::Rbp => "bp",
            RegisterName::Rsp => "sp",
            RegisterName::R8 => "r8",
            RegisterName::R9 => "r9",
            RegisterName::R10 => "r10",
            RegisterName::R11 => "r11",
            RegisterName::R12 => "r12",
            RegisterName::R13 => "r13",
            RegisterName::R14 => "r14",
            RegisterName::R15 => "r15",
            RegisterName::Invalid => "?",
        }
        .to_string()
    }

    fn get_end(&self) -> String {
        match RegisterType::from(&self.name) {
            RegisterType::GeneralPurpose => match self.optype {
                IOperatorSize::Byte => "b",
                IOperatorSize::Word => "w",
                IOperatorSize::Double => "d",
                IOperatorSize::Quad => "",
                _ => unreachable!(),
            },
            RegisterType::SpecialPurpose => match self.optype {
                IOperatorSize::Byte => "l",
                IOperatorSize::Void => unreachable!(),
                _ => "x",
            },
        }
        .to_string()
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "%{}{}{}",
            self.get_start(),
            self.get_middle(),
            self.get_end()
        )
    }
}
