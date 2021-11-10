#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParameterId(pub usize);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VariableId(pub usize);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SymbolName(pub String);

impl From<ParameterId> for SymbolId {
    fn from(id: ParameterId) -> Self {
        SymbolId(id.0)
    }
}

impl From<SymbolId> for FunctionId {
    fn from(id: SymbolId) -> Self {
        FunctionId(id.0)
    }
}
