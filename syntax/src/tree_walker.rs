use crate::{
    error::SyntaxBuilderError,
    id::{SymbolId, SymbolName},
    node::{ConstantNodeValue, NodeType, SyntaxNode},
    symbol::{ReturnType, Symbol, SymbolType},
    visitor::Visitor,
};
use lexical::{ParseNode, ParseTree, Rule};

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
        mut parse_tree: ParseTree,
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
                log::trace!("Before loop VAR_DECL");
                loop {
                    match self.walk_tree(nodes.next(), visitor)? {
                        ParserValue::Name(name) => {
                            log::trace!("Got name {}", name);
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
            Rule::expression => Ok(ParserValue::Node(SyntaxNode::Constant {
                node_type: NodeType::Error,
                return_type: ReturnType::Error,
                value: ConstantNodeValue::ErrorMessage(
                    "Expressions are not yet implemented".into(),
                ),
            })),
            Rule::var => {
                let var = SymbolName::from(parse_node.as_str());
                Ok(ParserValue::Name(var))
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

    // fn handle_expression(&self, node: ParseNode) {
    //     for pair in node.into_inner() {
    //         log::trace!("{} {:?}", pair.as_str(), pair.as_rule());
    //         self.handle_expression(pair);
    //     }
    // }

    // fn get_function_parts<'a>(
    //     &self,
    //     func: ParseNode<'a>,
    //     visitor: &mut Visitor,
    // ) -> Result<FunctionParts<'a>, SyntaxBuilderError> {
    //     let mut func_components = func.into_inner();
    //     let type_specifier = loop {
    //         let node = func_components.next().ok_or_else(|| {
    //             SyntaxBuilderError::from("Cannot get type_specifier for function")
    //         })?;
    //         match self.walk_tree(node, visitor)? {
    //             Some(snode) => break snode,
    //             None => continue,
    //         }
    //     };

    //     // .ok_or("Missing function return type specifier") ?
    //     // .as_str()
    //     // .into();
    //     let ident = func_components
    //         .next()
    //         .ok_or("Missing function identifier")?
    //         .as_str()
    //         .into();
    //     let params = func_components.next().ok_or("Missing function params")?;
    //     let func_body = func_components.next().ok_or("Missing function body")?;
    //     Ok(FunctionParts {
    //         return_type: type_specifier,
    //         name: ident,
    //         params,
    //         body: func_body,
    //     })
    // }

    // fn handle_param_decl(
    //     &mut self,
    //     params: ParseNode,
    //     visitor: &mut Visitor,
    // ) -> Result<(), SyntaxBuilderError> {
    //     for pair in params.into_inner() {
    //         if ReturnType::from(pair.as_str()) == ReturnType::Void {
    //             break;
    //         }
    //         let (type_spec, ident) = pair
    //             .into_inner()
    //             .collect_tuple()
    //             .expect("Invariant violated: No pair of type specifier & identifier");
    //         let type_spec = ReturnType::from(type_spec.as_str());
    //         let ident = SymbolName::from(ident.as_str());
    //         visitor.visit_param_decl(ident, type_spec, self.current_line)?;
    //     }
    //     Ok(())
    // }
}
