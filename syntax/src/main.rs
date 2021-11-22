mod builder;
mod error;
mod id;
mod node;
mod scope;
mod symbol;
mod symbol_table;
mod syntax_tree;
mod tree_walker;
mod visitor;

use general::logging;

#[derive(Debug)]
pub enum Node {
    Unary {
        child: Option<Box<Node>>,
        value: String,
    },
    Binary {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        value: String,
    },
    Empty,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logger();

    let mut nodes = vec![
        Node::Unary {
            value: "RetStmt".to_string(),
            child: Some(Box::new(Node::Unary {
                child: None,
                value: "Num 0".to_string(),
            })),
        },
        Node::Binary {
            value: "FuncCall".to_string(),
            left: Some(Box::new(Node::Unary {
                child: None,
                value: "Symbol(0)".to_string(),
            })),
            right: None,
        },
    ];

    // let mut previous = iter.next().unwrap();
    let mut stmt_node = Node::Binary {
        value: "StmtList".to_string(),
        left: None,
        right: None,
    };
    for node in nodes.drain(..).rev() {
        if let Node::Binary { ref mut left, .. } = stmt_node {
            *left = Some(Box::new(node));
        }

        stmt_node = Node::Binary {
            value: "StmtList".to_string(),
            left: None,
            right: Some(Box::new(stmt_node)),
        }
    }
    log::trace!("{:?}", stmt_node);
    // let input = std::fs::read_to_string("test.c")?;
    // let parse_tree = lexical::parse(&input)?;
    // syntax::generate(parse_tree).and(Ok(()))
    Ok(())
}
