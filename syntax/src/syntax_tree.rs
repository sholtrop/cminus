use std::collections::HashMap;

use crate::{id::FunctionId, node::Node};

pub struct FunctionRoot {
    name: String,
    root: Option<Node>,
}

pub struct SyntaxTree {
    pub functions: HashMap<FunctionId, FunctionRoot>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}
