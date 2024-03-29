use ptree::PrintConfig;
use std::collections::HashMap;
use std::fmt;

use crate::node::TESTING;
use crate::{
    id::{SymbolId, SymbolName},
    node::PostorderIter,
    node::SyntaxNode,
    node::SyntaxNodeBox,
};

pub enum NodeChildren {
    Unary { child: SyntaxNode },
    Binary { left: SyntaxNode, right: SyntaxNode },
}

pub struct FunctionRoot {
    pub name: SymbolName,
    pub tree: Option<SyntaxNodeBox>,
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

    pub fn get_root(&self, id: &SymbolId) -> Option<SyntaxNodeBox> {
        self.functions.get(id)?.tree.clone()
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

    pub fn postorder_traverse(&mut self) -> Vec<PostorderIter> {
        let mut iters = vec![];
        for (_, f) in self.functions.iter().filter(|(id, _)| !id.is_builtin()) {
            let iter = SyntaxNode::postorder(f.tree.as_ref().unwrap());
            iters.push(iter)
        }
        iters
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
                ptree::write_tree_with(&*tree.borrow(), &mut buff, &SyntaxTree::get_print_config())
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
