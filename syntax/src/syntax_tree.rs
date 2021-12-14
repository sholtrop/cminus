use ptree::PrintConfig;
use std::collections::HashMap;
use std::{
    fmt,
    io::{BufReader, BufWriter},
};

use crate::node::TESTING;
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

pub const BUILT_INS: [&str; 4] = [
    "writeinteger",
    "writeunsigned",
    "readinteger",
    "readunsigned",
];

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn get_root(&self, id: &SymbolId) -> Option<&SyntaxNode> {
        self.functions.get(id)?.tree.as_ref()
    }

    /// For tests
    pub fn get_func_by_name(&self, func: &str) -> Option<&FunctionRoot> {
        self.functions.values().find(|f| f.name.0 == func)
    }

    pub fn get_print_config() -> PrintConfig {
        PrintConfig {
            indent: 5,
            ..Default::default()
        }
    }
}

impl fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in self.functions.values() {
            if BUILT_INS.contains(&func.name.0.as_str()) {
                continue;
            }
            if !*TESTING {
                writeln!(f, "function `{}`", func.name)?;
            }
            if let Some(tree) = &func.tree {
                let mut buff = vec![];
                ptree::write_tree_with(tree, &mut buff, &SyntaxTree::get_print_config())
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
