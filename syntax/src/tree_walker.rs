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

impl From<SyntaxBuilderError> for ParserValue {
    fn from(e: SyntaxBuilderError) -> Self {
        Self::Node(SyntaxNode::create_error(e.to_string()))
    }
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
            if let ParserValue::Node(err) = self.walk_tree(Some(rule), visitor) {
                log::error!("{}", err);
            }
        }
        Ok(())
    }

    fn walk_tree(&mut self, parse_node: Option<ParseNode>, visitor: &mut Visitor) -> ParserValue {
        if parse_node.is_none() {
            return ParserValue::End;
        }
        let parse_node = parse_node.unwrap();
        log::trace!("{:?}", parse_node.as_rule());
        match parse_node.as_rule() {
            Rule::program => {
                visitor.program_start();
                for node in parse_node.into_inner() {
                    self.walk_tree(Some(node), visitor);
                }
                ParserValue::End
            }
            Rule::fn_declaration => {
                let mut nodes = parse_node.into_inner();
                let return_type = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => panic!("Expected function return type"),
                    };
                };
                let name = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => panic!("Expected function name"),
                    };
                };
                let id = match visitor.visit_func_start(Symbol {
                    name,
                    return_type,
                    symbol_type: SymbolType::Function,
                    line: self.current_line,
                }) {
                    Ok(id) => id,
                    Err(e) => {
                        return e.into();
                    }
                };
                // Param declaration is handled in Rule::Param
                // We do need to check if that worked correctly, but don't need the return value
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Ids(ids) => {
                            log::trace!("Func parameters added: {:?}", ids);
                            break;
                        }
                        ParserValue::None => break,
                        ParserValue::Skip => continue,
                        _ => panic!("Expected function parameters"),
                    };
                }
                let func_body = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(body) => break body,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected function body"), // return Err("Expected function body".into()),
                    };
                };
                if let Err(e) = visitor.visit_func_end(&id, func_body) {
                    e.into()
                } else {
                    ParserValue::Skip
                }
            }
            Rule::var_declaration => {
                let mut nodes = parse_node.into_inner();
                let type_spec = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected type specifier"),
                    };
                };
                self.current_decl_type = Some(type_spec);
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Nodes(nodes) => return ParserValue::Nodes(nodes),
                        ParserValue::None => return ParserValue::None,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected var names"),
                    }
                }
            }
            Rule::var_decl_list => {
                let mut nodes = parse_node.into_inner();
                let mut assignments = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        // Declaration with assignment
                        ParserValue::Node(n) => assignments.push(n),
                        ParserValue::End => break,
                        // Only a declaration, no assignment
                        ParserValue::None | ParserValue::Skip | ParserValue::Id(_) => continue,
                        _ => unreachable!("Did not find var_decl nodes"),
                    }
                }
                self.current_decl_type = None;
                ParserValue::Nodes(assignments)
            }
            Rule::var_decl_maybe_init => {
                let decl_type = self
                    .current_decl_type
                    .expect("No type specifier set for declaration list");
                let mut nodes = parse_node.into_inner();
                let mut current_id: Option<SyntaxNode> = None;
                let mut assignment: Option<SyntaxNode> = None;
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Name(name) => {
                            let id =
                                match visitor.visit_var_decl(name, decl_type, self.current_line) {
                                    Ok(id) => id,
                                    Err(e) => return e.into(),
                                };
                            current_id = Some(SyntaxNode::Symbol {
                                node_type: NodeType::Id,
                                return_type: decl_type,
                                symbol_id: id,
                            });
                        }
                        ParserValue::Node(node) => {
                            if let Some(var) = current_id.clone() {
                                assignment = match visitor.visit_assignment(var, node) {
                                    Ok(n) => Some(n),
                                    Err(e) => return e.into(),
                                }
                            } else {
                                panic!("Invariant violated: Assignment expression without id to assign to")
                            }
                        }
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => unreachable!("Expected var identifier / assign expression"),
                    }
                }
                if let Some(assignment) = assignment {
                    ParserValue::Node(assignment)
                } else {
                    ParserValue::None
                }
            }
            Rule::array_decl | Rule::array_parameter => {
                let is_param = parse_node.as_rule() == Rule::array_parameter;
                let mut nodes = parse_node.into_inner();
                let mut ident: Option<SymbolName> = None;
                let mut size: Option<SyntaxNode> = None;
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::ReturnType(rt) => self.current_decl_type = Some(rt),
                        ParserValue::Name(name) => ident = Some(name),
                        ParserValue::Node(n) => size = Some(n),
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => unreachable!("Expected array type, ident or size"),
                    }
                }
                if let Some(ident) = ident {
                    if let Some(size) = size {
                        if is_param {
                            return match visitor.visit_array_param_decl(
                                ident,
                                size,
                                self.current_decl_type.expect("No declaration type set"),
                                self.current_line,
                            ) {
                                Ok(id) => ParserValue::Id(id),
                                Err(e) => ParserValue::Node(e.into()),
                            };
                        } else {
                            return match visitor.visit_array_decl(
                                ident,
                                size,
                                self.current_decl_type.expect("No declaration type set"),
                                self.current_line,
                            ) {
                                Ok(id) => ParserValue::Id(id),
                                Err(e) => ParserValue::Node(e.into()),
                            };
                        }
                    }
                }
                ParserValue::Node(SyntaxNode::create_error("Missing array ident or size"))
            }
            Rule::formal_parameters => {
                let mut nodes = parse_node.into_inner();
                let mut params = vec![];
                loop {
                    // TODO: formal_parameters changed into normal/array param, change this loop as well
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::ReturnType(void) => {
                            if void == ReturnType::Void {
                                break;
                            } else {
                                panic!("Expected identifier as name for param",);
                            }
                        }
                        ParserValue::Id(param_id) => params.push(param_id),
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => {
                            panic!("Expected identifier as name for param");
                        }
                    };
                }
                if params.is_empty() {
                    ParserValue::None
                } else {
                    ParserValue::Ids(params)
                }
            }
            Rule::parameter => {
                let mut nodes = parse_node.into_inner();
                let type_spec = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::ReturnType(rt) => break rt,
                        ParserValue::Skip => continue,
                        _ => panic!("Expected type specifier for param"),
                    };
                };
                let ident = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => panic!("Expected identifier as name for param"),
                    };
                };
                let id = match visitor.visit_param_decl(ident, type_spec, self.current_line) {
                    Ok(id) => id,
                    Err(e) => return e.into(),
                };
                ParserValue::Id(id)
            }
            Rule::void => ParserValue::ReturnType(ReturnType::Void),
            Rule::compound_stmt => {
                visitor.add_local_scope();
                let mut nodes = parse_node.into_inner();
                let mut statements = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(node) => statements.push(node),
                        ParserValue::Nodes(mut nodes) => statements.append(&mut nodes),
                        ParserValue::None => continue,
                        ParserValue::Skip => continue,
                        ParserValue::End => break,
                        _ => unreachable!("Expected statement"),
                    };
                }
                let root = visitor.visit_statement_list(statements);
                visitor.leave_local_scope();
                ParserValue::Node(root)
            }
            Rule::function_call => {
                let mut nodes = parse_node.into_inner();
                let func_name = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Name(name) => break name,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected identifier as function name"),
                    };
                };
                let params = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Nodes(n) => break n,
                        ParserValue::End => break vec![],
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected actual parameters"),
                    };
                };
                let func_call_node = match visitor.visit_func_call(&func_name, params) {
                    Ok(n) => n,
                    Err(e) => return e.into(),
                };
                ParserValue::Node(func_call_node)
            }
            Rule::actual_parameters => {
                let mut nodes = parse_node.into_inner();
                let mut params = vec![];
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(node) => params.push(node),
                        ParserValue::Skip => continue,
                        ParserValue::End | ParserValue::None => break,
                        _ => unreachable!("Expected expression"),
                    };
                }
                ParserValue::Nodes(params)
            }
            Rule::type_specifier => {
                let rtype: ReturnType = parse_node.as_str().into();
                ParserValue::ReturnType(rtype)
            }
            Rule::number => {
                let parse_node = parse_node.as_str().to_string();
                let res = visitor
                    .visit_number(parse_node)
                    .unwrap_or_else(SyntaxNode::from);
                ParserValue::Node(res)
            }
            Rule::ident => {
                let ident = SymbolName::from(parse_node.as_str());
                ParserValue::Name(ident)
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
                ParserValue::Node(node)
            }
            Rule::return_stmt => {
                let mut nodes = parse_node.into_inner();
                let return_exp = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break Some(n),
                        ParserValue::End => break None,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected return expression"),
                    };
                };
                let return_node = visitor.visit_return(return_exp);
                ParserValue::Node(return_node)
            }
            Rule::selection_stmt => {
                let mut nodes = parse_node.into_inner();
                let if_exp = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected expression node for if-statement test"),
                    }
                };
                let if_body = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected statement node for if-statement body"),
                    }
                };
                let else_body = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break Some(n),
                        ParserValue::End => break None,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected statement node for if-statement body"),
                    }
                };
                let if_node = match visitor.visit_if(if_exp, if_body, else_body) {
                    Ok(n) => n,
                    Err(e) => return e.into(),
                };
                ParserValue::Node(if_node)
            }
            Rule::expression => {
                log::trace!("Enter expression `{}`", parse_node.as_str());
                let mut list = parse_node
                    .into_inner()
                    .filter_map(|node| {
                        let res = self.walk_tree(Some(node), visitor);
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

                    highest_prec = list.remove(highest_idx).unwrap();
                    log::trace!(
                        "Highest idx {} | Highest prec {}",
                        highest_idx,
                        highest_prec
                    );
                    let new_left = list.remove(highest_idx - 1).unwrap();
                    let new_right = list.remove(highest_idx - 1).unwrap();
                    if let Err(e) = visitor.visit_binary(new_left, &mut highest_prec, new_right) {
                        return e.into();
                    }
                    list.insert(highest_idx - 1, highest_prec);
                }
                ParserValue::Node(list.pop_front().unwrap())
            }
            Rule::assignment => {
                let mut nodes = parse_node.into_inner();
                let lvar = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected lvariable"),
                    }
                };
                let expr = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected assignment expression"),
                    }
                };
                let assignment = match visitor.visit_assignment(lvar, expr) {
                    Ok(n) => n,
                    Err(e) => return e.into(),
                };
                ParserValue::Node(assignment)
            }
            Rule::lvar | Rule::rvar => {
                let mut nodes = parse_node.into_inner();
                let mut access_exp: Option<SyntaxNode> = None;
                let mut name: Option<SymbolName> = None;
                loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Name(n) => name = Some(n),
                        ParserValue::Node(exp) => access_exp = Some(exp),
                        ParserValue::End => break,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected identifier"),
                    }
                }
                let name = name.unwrap();
                let id_node = if let Some(array_access) = access_exp {
                    visitor.visit_array_access(&name, array_access)
                } else {
                    visitor.visit_variable(&name)
                };
                match id_node {
                    Ok(n) => ParserValue::Node(n),
                    Err(e) => e.into(),
                }
            }
            Rule::iteration_stmt => {
                let mut nodes = parse_node.into_inner();
                let condition = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(exp) => break exp,
                        ParserValue::Skip => continue,
                        _ => {
                            unreachable!("Expected condition expression for while loop")
                        }
                    }
                };
                let statement = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(stmt) => break stmt,
                        ParserValue::Skip => continue,
                        _ => {
                            unreachable!("Expected statement for while loop")
                        }
                    }
                };
                match visitor.visit_while(condition, statement) {
                    Ok(n) => ParserValue::Node(n),
                    Err(e) => e.into(),
                }
            }
            Rule::unary => {
                let mut nodes = parse_node.into_inner();
                let unary_op = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected node"),
                    }
                };
                let unary_child = loop {
                    match self.walk_tree(nodes.next(), visitor) {
                        ParserValue::Node(n) => break n,
                        ParserValue::Skip => continue,
                        _ => unreachable!("Expected node"),
                    }
                };
                match visitor.visit_unary(unary_op, unary_child) {
                    Ok(n) => ParserValue::Node(n),
                    Err(e) => e.into(),
                }
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
                ParserValue::Node(return_val)
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
                ParserValue::Skip
            }
            Rule::EOI => ParserValue::Skip,
            _ => {
                unreachable!(
                    "Unimplemented rule `{:?}`:\n{}. {}",
                    parse_node.as_rule(),
                    self.current_line,
                    parse_node.as_str()
                );
            }
        }
    }
}
