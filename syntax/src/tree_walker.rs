use std::collections::VecDeque;

use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    node::{ConstantNodeValue, NodeType, SyntaxNode},
    symbol::{ReturnType, Symbol, SymbolType},
    visitor::Visitor,
};
use itertools::Itertools;
use lexical::{ParseNode, ParseTree, Rule};

#[derive(Debug)]
pub enum ParserValue {
    Symbol(Symbol),
    Symbols(Vec<Symbol>),
    Node(SyntaxNode),
    Nodes(Vec<SyntaxNode>),
    Name(SymbolName),
    Id(SymbolId),
    Ids(Vec<SymbolId>),
    ReturnType(ReturnType),
    Skip,
    None,
    End,
}

pub struct TreeWalker {
    current_line: usize,
    current_decl_type: Option<ReturnType>,
}

impl TreeWalker {
    pub fn new() -> Self {
        Self {
            current_line: 1,
            current_decl_type: None,
        }
    }

    pub fn construct_syntax_tree(
        &mut self,
        parse_tree: ParseTree,
        visitor: &mut Visitor,
    ) -> Result<(), SyntaxBuilderError> {
        for rule in parse_tree {
            self.walk_tree(Some(rule), visitor)?;
        }
        Ok(())
    }

    fn walk_tree(
        &mut self,
        parse_node: Option<ParseNode>,
        visitor: &mut Visitor,
    ) -> Result<ParserValue, SyntaxBuilderError> {
        if parse_node.is_none() {
            return Ok(ParserValue::End);
        }
        let parse_node = parse_node.unwrap();
        log::trace!("{:?}", parse_node.as_rule());
        match parse_node.as_rule() {
            Rule::program => {
                visitor.program_start();
                for node in parse_node.into_inner() {
                    self.walk_tree(Some(node), visitor)?;
                }
                Ok(ParserValue::End)
            }
            Rule::fn_declaration => {
                let mut nodes = parse_node.into_inner();
                let return_type = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected function return type")),
                    };
                };
                let name = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected function name")),
                    };
                };
                let id = visitor.visit_func_start(Symbol {
                    name,
                    return_type,
                    symbol_type: SymbolType::Function,
                    line: self.current_line,
                })?;
                // Param declaration is handled in Rule::Param
                // We do need to check if that worked correctly, but don't need the return value
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Ids(_) | ParserValue::None => break,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected function parameters")),
                    };
                }
                let func_body = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Node(body) => break body,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected function body")),
                    };
                };
                visitor.visit_func_end(&id, func_body)?;
                Ok(ParserValue::Skip)
            }
            Rule::var_declaration => {
                let mut nodes = parse_node.into_inner();
                let type_spec = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected type specifier")),
                    };
                };
                self.current_decl_type = Some(type_spec);
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Nodes(nodes) => return Ok(ParserValue::Nodes(nodes)),
                        ParserValue::None => return Ok(ParserValue::None),
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected var names")),
                    }
                }
            }
            Rule::var_decl_list => {
                let mut nodes = parse_node.into_inner();
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::None => return Ok(ParserValue::None),
                        ParserValue::Nodes(n) => return Ok(ParserValue::Nodes(n)),
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Did not find var_decl nodes")),
                    }
                }
            }
            Rule::var_decl_maybe_init => {
                let decl_type = self.current_decl_type.take().ok_or_else(|| {
                    SyntaxBuilderError::from("No type specifier set for declaration list")
                })?;
                let mut nodes = parse_node.into_inner();
                let mut current_name: Option<SymbolName> = None;
                let mut assignments = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Name(name) => {
                            current_name = Some(name.clone());
                            visitor.visit_var_decl(name, decl_type, self.current_line)?;
                        }
                        ParserValue::Node(exp) => {
                            if let Some(var) = current_name.clone() {
                                let assignment = visitor.visit_assignment(var, exp)?;
                                assignments.push(assignment);
                            } else {
                                log::warn!(
                                    "Could not visit assignment because `current_name` was None"
                                );
                            }
                        }
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => {
                            return Err(SyntaxBuilderError::from(
                                "Expected var identifier / assign expression",
                            ))
                        }
                    }
                }
                if assignments.is_empty() {
                    Ok(ParserValue::None)
                } else {
                    Ok(ParserValue::Nodes(assignments))
                }
            }
            Rule::formal_parameters => {
                let mut nodes = parse_node.into_inner();
                let mut params = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::None => break,
                        ParserValue::Id(param_id) => params.push(param_id),
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => {
                            return Err(SyntaxBuilderError::from(
                                "Expected identifier as name for param",
                            ));
                        }
                    };
                }
                if params.is_empty() {
                    Ok(ParserValue::None)
                } else {
                    Ok(ParserValue::Ids(params))
                }
            }
            Rule::parameter => {
                let mut nodes = parse_node.into_inner();
                let type_spec = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => {
                            return Err(SyntaxBuilderError::from(
                                "Expected type specifier for param",
                            ))
                        }
                    };
                };
                let ident = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => {
                            return Err(SyntaxBuilderError::from(
                                "Expected identifier as name for param",
                            ))
                        }
                    };
                };
                let id = visitor.visit_param_decl(ident, type_spec, self.current_line)?;
                Ok(ParserValue::Id(id))
            }
            Rule::void => Ok(ParserValue::None),
            Rule::compound_stmt => {
                visitor.add_local_scope();
                let mut nodes = parse_node.into_inner();
                let mut statements = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Node(node) => statements.push(node),
                        ParserValue::Nodes(mut nodes) => statements.append(&mut nodes),
                        ParserValue::None => continue,
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => return Err(SyntaxBuilderError::from("Expected statement")),
                    };
                }
                let root = visitor.visit_statement_list(statements);
                visitor.leave_local_scope();
                Ok(ParserValue::Node(root))
            }
            Rule::function_call => {
                let mut nodes = parse_node.into_inner();
                let params = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Nodes(n) => break n,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected actual parameters")),
                    };
                };
                let func_name = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => {
                            return Err(SyntaxBuilderError::from(
                                "Expected identifier as function name",
                            ))
                        }
                    };
                };
                let func_call_node = visitor.visit_func_call(&func_name, params)?;
                Ok(ParserValue::Node(func_call_node))
            }
            Rule::actual_parameters => {
                let mut nodes = parse_node.into_inner();
                let mut params = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Node(node) => params.push(node),
                        ParserValue::Skip => continue,
                        ParserValue::End | ParserValue::None => break,
                        _ => return Err(SyntaxBuilderError::from("Expected expression")),
                    };
                }
                Ok(ParserValue::Nodes(params))
            }
            Rule::type_specifier => {
                let rtype: ReturnType = parse_node.as_str().into();
                Ok(ParserValue::ReturnType(rtype))
            }
            Rule::number => {
                let parse_node = parse_node.as_str().to_string();
                let res = visitor
                    .visit_number(parse_node)
                    .unwrap_or_else(SyntaxNode::from);
                Ok(ParserValue::Node(res))
            }
            Rule::ident => {
                let ident = SymbolName::from(parse_node.as_str());
                Ok(ParserValue::Name(ident))
            }
            Rule::infix_op => {
                use NodeType::*;
                use Rule::*;
                let op = parse_node.into_inner().next().unwrap();
                let node = SyntaxNode::Binary {
                    node_type: match op.as_rule() {
                        add => Add,
                        sub => Sub,
                        mul => Mul,
                        div => Div,
                        modulo => Mod,
                        and => And,
                        or => Or,
                        eq => RelEqual,
                        neq => RelNotEqual,
                        lt => RelLT,
                        lte => RelLTE,
                        gt => RelGT,
                        gte => RelGTE,
                        _ => unreachable!(),
                    },
                    return_type: ReturnType::Unknown,
                    left: None,
                    right: None,
                };
                Ok(ParserValue::Node(node))
            }
            Rule::expression => {
                log::trace!("Enter expression `{}`", parse_node.as_str());
                let mut list = parse_node
                    .into_inner()
                    .filter_map(|node| {
                        let res = self.walk_tree(Some(node), visitor);
                        if let Err(e) = res {
                            return Some(SyntaxNode::from(e));
                        }
                        let res = res.unwrap();
                        match res {
                            ParserValue::Node(n) => Some(n),
                            ParserValue::Skip => None,
                            _ => panic!("Expected syntax node"),
                        }
                    })
                    .collect::<VecDeque<SyntaxNode>>();
                while list.len() != 1 {
                    let mut highest_prec;
                    let highest_idx = list
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| idx % 2 == 1)
                        .position_max_by(|(_, a), (_, b)| {
                            a.precedence().unwrap().cmp(&b.precedence().unwrap())
                        })
                        .unwrap()
                        * 2
                        + 1;

                    // log::trace!("{:?}", list);
                    highest_prec = list.remove(highest_idx).unwrap();
                    log::trace!(
                        "Highest idx {} | Highest prec {}",
                        highest_idx,
                        highest_prec
                    );
                    // log::trace!("{:?}", list);
                    let new_left = list.remove(highest_idx - 1).unwrap();
                    let new_right = list.remove(highest_idx - 1).unwrap();
                    visitor.visit_binary(new_left, &mut highest_prec, new_right)?;
                    list.insert(highest_idx - 1, highest_prec);
                }
                Ok(ParserValue::Node(list.pop_front().unwrap()))
            }
            Rule::var => {
                let var = SymbolName::from(parse_node.as_str());
                Ok(ParserValue::Name(var))
            }
            Rule::unary => {
                let mut nodes = parse_node.into_inner();
                let unary_op = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected node")),
                    }
                };
                let unary_child = loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => return Err(SyntaxBuilderError::from("Expected node")),
                    }
                };
                let result = visitor.visit_unary(unary_op, unary_child)?;
                Ok(ParserValue::Node(result))
            }
            Rule::unary_op => {
                let return_val = SyntaxNode::Unary {
                    child: None,
                    return_type: ReturnType::Unknown,
                    node_type: match parse_node.as_str() {
                        "-" => NodeType::SignMinus,
                        "+" => NodeType::SignPlus,
                        "!" => NodeType::Not,
                        _ => unreachable!(),
                    },
                };
                Ok(ParserValue::Node(return_val))
            }
            Rule::COMMENT | Rule::WHITESPACE => {
                // We can never enter the newline rule, so we manually count newlines in whitespace/comments
                let newlines = parse_node.as_str().chars().fold(0, |n, c| {
                    if c == '\r' || c == '\n' {
                        n + 1
                    } else {
                        n
                    }
                });
                self.current_line += newlines;
                Ok(ParserValue::Skip)
            }
            Rule::EOI => Ok(ParserValue::Skip),
            _ => {
                log::warn!(
                    "Unimplemented rule `{:?}`:\n{}. {}",
                    parse_node.as_rule(),
                    self.current_line,
                    parse_node.as_str()
                );
                Ok(ParserValue::Skip)
            }
        }
    }
}

// fn eval_expression(exp: ParseNode, prec_climber: &PrecClimber<Rule>) -> SyntaxNode {
// let exp = exp.into_inner();
// prec_climber.climb(
//     exp,
//     |node| {
//         let snode = loop {};
//     },
//     |lhs: SyntaxNode, op: ParseNode, rhs: SyntaxNode| match op.as_rule() {
//         _ => {
//             log::trace!("{} {:?} {}", lhs, op.as_rule(), rhs);
//             SyntaxNode::Empty
//         }
//     },
// )
// }
