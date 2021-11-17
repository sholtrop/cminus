use std::{collections::HashMap, fmt, io::BufWriter, vec};

use crate::{
    id::{SymbolId, SymbolName},
    node::Node,
};
use std::io::Write;

pub enum NodeChildren {
    Unary { child: Node },
    Binary { left: Node, right: Node },
}

pub struct FunctionRoot {
    pub name: SymbolName,
    pub root: Option<Node>,
}

pub struct SyntaxTree {
    pub functions: HashMap<SymbolId, FunctionRoot>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn get_root(&self, id: &SymbolId) -> Option<&Node> {
        self.functions.get(id)?.root.as_ref()
    }
}

impl fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = BufWriter::new(vec![]);
        for func in self.functions.values() {
            if let Some(node) = &func.root {
                ptree::write_tree(node, &mut buff).map_err(|_| std::fmt::Error {})?;
            }
        }
        writeln!(
            f,
            "SYNTAX TREE DUMP:\n{}",
            String::from_utf8(buff.into_inner().unwrap()).unwrap()
        )
    }
}
