#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub struct SymbolId(pub usize);
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SymbolName(pub String);

impl From<&str> for SymbolName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl std::fmt::Display for SymbolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
