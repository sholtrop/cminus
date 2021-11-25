use std::{
    collections::HashMap,
    fmt,
    io::{BufReader, BufWriter},
};

use ptree::PrintConfig;

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
    pub tree: Option<SyntaxNode>,
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
        self.functions.get_mut(id)?.tree.as_mut()
    }
}

impl fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in self.functions.values() {
            writeln!(f, "function `{}`", func.name)?;
            if let Some(tree) = &func.tree {
                let mut buff = vec![];
                ptree::write_tree_with(
                    tree,
                    &mut buff,
                    &PrintConfig {
                        indent: 5,
                        ..Default::default()
                    },
                )
                .expect("Error printing tree");
                let tree = String::from_utf8(buff).expect("Utf8 error printing tree");
                writeln!(f, "{}", tree)?;
            } else {
                writeln!(f, "<no root yet>")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
