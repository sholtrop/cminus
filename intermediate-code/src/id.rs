use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ISymbolId(usize);

impl fmt::Display for ISymbolId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
