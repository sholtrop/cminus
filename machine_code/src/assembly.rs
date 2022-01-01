pub mod asm {
    use std::fmt;

    pub enum Op {
        Mov(IOperatorSize),
    }

    impl fmt::Display for Op {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Mov(s) => format!("mov{}", s),
                }
            )
        }
    }

    use intermediate_code::ioperator::IOperatorSize;
    use syntax::{ConstantNodeValue, SymbolId};

    use crate::register::Register;

    use self::Op::*;

    pub enum Src {
        None,
        Immediate(ConstantNodeValue),
        Register(Register),
        Global(SymbolId),
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
                }
            )
        }
    }

    pub enum Dest {
        None,
        Register(Register),
        MemAddress(String),
        Global(String),
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
                    Self::MemAddress(m) => m.to_string(),
                }
            )
        }
    }

    pub struct Instr(pub Op, pub Src, pub Dest);
    impl fmt::Display for Instr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Self(op, src, dst) = self;
            let size = "q"; // TODO: correct size
            writeln!(
                f,
                "\t{}{}{}{}",
                op,
                size,
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

    pub struct Label(pub String);

    impl Label {
        pub fn new(s: impl AsRef<str>) -> Self {
            Self(s.as_ref().to_string())
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
                        Directive::File(s) => format!(".file\t{}", s),
                        Directive::Def(s) => format!(".def\t{}", s),
                        Directive::Text => ".text".into(),
                        Directive::Ascii(s) => format!(".ascii\t{}", s),
                        Directive::Global(s) => format!(".globl\t{}", s),
                        Directive::Comm { name, size } =>
                            format!(".comm\tv{}, {}", name, usize::from(*size).to_string()),
                    }
                )
            )
        }
    }
}