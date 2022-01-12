pub mod asm {
    use std::fmt;

    pub enum Op {
        Mov(IOperatorSize),
        Push(IOperatorSize),
        Pop(IOperatorSize),
        Call,
        Ret,
        Setnz,
        Add(IOperatorSize),
        Comp(IOperatorSize),
        Setne,
        // Mov with sign-extension
        Movs(IOperatorSize, IOperatorSize),
        // Mov with zero-extension
        Movz(IOperatorSize, IOperatorSize),
        Sub(IOperatorSize),
    }

    impl fmt::Display for Op {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Mov(s) => format!("mov{}", s),
                    Push(s) => format!("push{}", s),
                    Pop(s) => format!("pop{}", s),
                    Call => "call".into(),
                    Ret => "ret".into(),
                    Setnz => "setnz".into(),
                    Add(s) => format!("add{}", s),
                    Comp(s) => format!("cmp{}", s),
                    Setne => "setne".into(),
                    Movs(to, from) => format!("movz{}{}", to, from),
                    Movz(to, from) => format!("movs{}{}", to, from),
                    Sub(s) => format!("sub{}", s),
                }
            )
        }
    }

    use intermediate_code::{ioperand::IOperand, ioperator::IOperatorSize};
    use syntax::ConstantNodeValue;

    use crate::{
        reg_alloc::{StackOffset, StoredLocation},
        register::Register,
    };

    use self::Op::*;

    #[derive(Clone)]
    pub enum Src {
        None,
        Immediate(ConstantNodeValue),
        Register(Register),
        Global(String),
        Label(String),
        Stack(StackOffset),
    }

    impl From<Register> for Src {
        fn from(reg: Register) -> Self {
            Self::Register(reg)
        }
    }

    impl fmt::Display for Src {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::None => "".into(),
                    Self::Global(id) => format!("v{}(%rip)", id),
                    Self::Immediate(i) => format!("${}", i),
                    Self::Register(r) => r.to_string(),
                    Self::Label(l) => l.to_string(),
                    Self::Stack(o) => format!("{}%rbp", o.0),
                }
            )
        }
    }

    #[derive(Clone)]
    pub enum Dest {
        None,
        Register(Register),
        Stack(StackOffset),
        Global(String),
        Label(String),
        Immediate(ConstantNodeValue),
    }

    impl From<&StoredLocation> for Dest {
        fn from(loc: &StoredLocation) -> Self {
            match loc {
                StoredLocation::Global(g) => Dest::Global(g.to_string()),
                StoredLocation::Reg(r) => Dest::Register(*r),
                StoredLocation::Stack(s) => Dest::Stack(*s),
            }
        }
    }

    impl From<Register> for Dest {
        fn from(reg: Register) -> Self {
            Self::Register(reg)
        }
    }

    impl fmt::Display for Dest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::None => "".into(),
                    Self::Register(r) => r.to_string(),
                    Self::Global(s) => format!("v{}(%rip)", s),
                    Self::Stack(offset) => format!("{}(%rsp)", offset.0),
                    Self::Label(l) => l.to_string(),
                    Self::Immediate(v) => v.to_string(),
                }
            )
        }
    }

    pub struct Instr(pub Op, pub Src, pub Dest);

    pub fn instr(op: impl Into<Op>, src: impl Into<Src>, dst: impl Into<Dest>) -> Instr {
        Instr(op.into(), src.into(), dst.into())
    }

    impl fmt::Display for Instr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Self(op, src, dst) = self;
            writeln!(
                f,
                "\t{}{}{}",
                op,
                match src {
                    Src::None => "".into(),
                    _ => format!("\t{}", self.1),
                },
                match dst {
                    Dest::None => "".into(),
                    _ => format!(", {}", self.2),
                }
            )
        }
    }

    #[derive(Clone)]
    pub struct Label(pub String);

    impl Label {
        pub fn new(name: impl ToString) -> Self {
            Self(name.to_string())
        }
    }

    impl fmt::Display for Label {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}:", self.0)
        }
    }
    pub enum Directive {
        File(String),
        Def(String),
        Text,
        Ascii(String),
        Global(String),
        Comm { name: String, size: IOperatorSize },
    }

    impl fmt::Display for Directive {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                format!(
                    "\t.{}\n",
                    match self {
                        Directive::File(s) => format!("file\t{}", s),
                        Directive::Def(s) => format!("def\t{}", s),
                        Directive::Text => "text".into(),
                        Directive::Ascii(s) => format!("ascii\t{}", s),
                        Directive::Global(s) => format!("globl\t{}", s),
                        Directive::Comm { name, size } =>
                            format!("comm\tv{}, {}", name, usize::from(*size).to_string()),
                    }
                )
            )
        }
    }
}
