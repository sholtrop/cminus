pub mod asm {
    use std::fmt;

    pub enum Op {
        Mov(IOperatorSize),
        Push(IOperatorSize),
        Pop(IOperatorSize),
        Call,
        Ret,
        Add(IOperatorSize),
        Comp(IOperatorSize),
        Setne,
        Test(IOperatorSize),

        // Mov with sign-extension
        Movs(IOperatorSize, IOperatorSize),

        // Mov with zero-extension
        Movz(IOperatorSize, IOperatorSize),
        Sub(IOperatorSize),
        Jmp, // unconditional jump
        Je,  // jump if ==
        Jne, // jump if !=
        Js,  // jump if negative
        Jns, // jump if nonnegative
        Jg,  // jump if > signed
        Jge, // jump if >= signed
        Jl,  // jump if < signed
        Jle, // jump if <= signed

        Ja,  // jump if > unsigned
        Jae, // jump if >= unsigned
        Jb,  // jump if < unsigned
        Jbe, // jump if <= unsigned

        SetE,  // set if ==
        SetNE, // set if !=
        SetS,  // set if negative
        SetNS, // set if nonnegative
        SetG,  // set if > signed
        SetGE, // set if >= signed
        SetL,  // set if < signed
        SetLE, // set if <= signed
        SetA,  // set if > unsigned
        SetAE, // set if >= unsigned
        SetB,  // set if < unsigned
        SetBE, // set if <= unsigned
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
                    Add(s) => format!("add{}", s),
                    Comp(s) => format!("cmp{}", s),
                    Setne => "setne".into(),
                    Test(s) => format!("test{}", s),
                    Movs(to, from) => format!("movz{}{}", to, from),
                    Movz(to, from) => format!("movs{}{}", to, from),
                    Sub(s) => format!("sub{}", s),
                    Jmp => "jmp".into(),
                    Jne => "jne".into(),
                    Je => "je".into(),
                    Jg => "jg".into(),
                    Jge => "jge".into(),
                    Jl => "jl".into(),
                    Jle => "jle".into(),
                    Js => "js".into(),
                    Jns => "jse".into(),
                    Ja => "ja".into(),
                    Jae => "jae".into(),
                    Jb => "jb".into(),
                    Jbe => "jbe".into(),
                    SetE => "sete".into(),
                    SetNE => "setne".into(),
                    SetS => "sets".into(),
                    SetNS => "setns".into(),
                    SetG => "setg".into(),
                    SetGE => "setge".into(),
                    SetL => "setl".into(),
                    SetLE => "setle".into(),
                    SetA => "seta".into(),
                    SetAE => "setae".into(),
                    SetB => "setb".into(),
                    SetBE => "setbe".into(),
                }
            )
        }
    }

    use intermediate_code::ioperator::IOperatorSize;
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
    impl From<&StoredLocation> for Src {
        fn from(loc: &StoredLocation) -> Self {
            match loc {
                StoredLocation::Global(g) => Src::Global(g.to_string()),
                StoredLocation::Reg(r) => Src::Register(*r),
                StoredLocation::Stack(s) => Src::Stack(*s),
            }
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
                    Self::Immediate(v) => format!("${}", v),
                }
            )
        }
    }

    pub struct Instr(pub Op, pub Src, pub Dest);

    pub fn instr(op: impl Into<Op>, src: impl Into<Src>, dst: impl Into<Dest>) -> Instr {
        Instr(op.into(), src.into(), dst.into())
    }

    /// For instructions with two source operands rather than source + dest.
    pub fn instr2(op: impl Into<Op>, src1: impl Into<Src>, src2: impl Into<Src>) -> Instr {
        let src2 = match src2.into() {
            Src::Global(g) => Dest::Global(g),
            Src::Immediate(i) => Dest::Immediate(i),
            Src::Label(l) => Dest::Label(l),
            Src::Register(r) => Dest::Register(r),
            Src::Stack(s) => Dest::Stack(s),
            Src::None => Dest::None,
        };
        Instr(op.into(), src1.into(), src2)
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
