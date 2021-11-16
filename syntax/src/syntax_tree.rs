use std::collections::HashMap;

use crate::{
    id::{SymbolId, SymbolName},
    node::{Node, NodeType},
    symbol::ReturnType,
};

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

    // pub fn create_parent_node(
    //     node_type: NodeType,
    //     ret_type: ReturnType,
    //     children: NodeChildren,
    // ) -> Node {
    //     Node::Empty
    // }

    // pub fn create_id_node(id: SymbolId, ret_type: ReturnType) -> Node {
    //     Node::Empty
    // }

    // pub fn create_leaf_node() -> Node {
    //     Node::Empty
    // }
}
