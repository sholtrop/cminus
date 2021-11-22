use general::tree::ArenaTree;
use std::{collections::HashMap, fmt};

use crate::{
    id::{SymbolId, SymbolName},
    node::SyntaxNode,
};

pub enum NodeChildren {
    Unary { child: SyntaxNode },
    Binary { left: SyntaxNode, right: SyntaxNode },
}

pub struct FunctionRoot {
    pub name: SymbolName,
    pub tree: Option<ArenaTree<SyntaxNode>>,
}

#[derive(Default)]
pub struct SyntaxTree {
    pub functions: HashMap<SymbolId, FunctionRoot>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn get_root(&mut self, id: &SymbolId) -> Option<&mut SyntaxNode> {
        self.functions.get_mut(id)?.tree.as_mut()?.get_root_mut()
    }
}

impl fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in self.functions.values() {
            writeln!(f, "function `{}`", func.name)?;
            if let Some(tree) = &func.tree {
                writeln!(f, "{}", tree)?;
            } else {
                writeln!(f, "<no root yet>")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
