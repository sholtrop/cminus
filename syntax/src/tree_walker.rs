use crate::{
    error::SyntaxBuilderError,
    id::SymbolName,
    node::{NodeType, SyntaxNode},
    symbol::ReturnType,
    visitor::Visitor,
};
use itertools::Itertools;
use lexical::{ParseNode, ParseTree, Rule};

struct FunctionParts<'a> {
    pub return_type: String,
    pub name: String,
    pub params: ParseNode<'a>,
    pub body: ParseNode<'a>,
}

pub struct TreeWalker {
    current_line: usize,
}

impl TreeWalker {
    pub fn new() -> Self {
        Self { current_line: 1 }
    }

    pub fn construct_syntax_tree(
        &mut self,
        parse_tree: ParseTree,
        visitor: &mut Visitor,
    ) -> Result<(), SyntaxBuilderError> {
        for node in parse_tree {
            self.walk_tree(node, visitor)?;
        }
        Ok(())
    }

    fn walk_tree(
        &mut self,
        parse_node: ParseNode,
        visitor: &mut Visitor,
    ) -> Result<Option<SyntaxNode>, SyntaxBuilderError> {
        match parse_node.as_rule() {
            Rule::fn_declaration => {
                let FunctionParts {
                    return_type,
                    name,
                    params,
                    body,
                } = self.get_function_parts(parse_node)?;
                log::trace!(
                    "Function decl:\nret: {}\nname: {}\nparams: {}\n",
                    return_type,
                    name,
                    params.as_str(),
                );
                let id = visitor
                    .visit_func_start(SymbolName(name), ReturnType::from(return_type.as_str()))?;
                self.handle_param_decl(params, visitor)?;
                let func_body = self.walk_tree(body, visitor)?.unwrap_or(SyntaxNode::Empty);
                visitor.visit_func_end(&id, func_body);
                Ok(None)
            }
            Rule::compound_stmt => {
                log::trace!("compound statement");
                visitor.add_local_scope();
                let statements: Vec<SyntaxNode> = parse_node
                    .into_inner()
                    .into_iter()
                    .map(|node| {
                        self.walk_tree(node, visitor)
                            .unwrap_or_else(|e| {
                                log::warn!("Error in compound_stmt {}", e);
                                Some(SyntaxNode::create_error(e))
                            })
                            .unwrap_or_else(|| {
                                SyntaxNode::create_error("Node was not parsed to statement")
                            })
                    })
                    .collect();
                let root = visitor.visit_statement_list(statements)?;
                visitor.leave_local_scope();
                Ok(Some(root))
            }
            Rule::function_call => {
                log::trace!("Function call: {}", parse_node.as_str());
                let (func_name, params) =
                    parse_node.into_inner().collect_tuple().ok_or_else(|| {
                        SyntaxBuilderError::new("Could not split func call into name and params")
                    })?;
                let func_name = SymbolName(func_name.to_string());
                let params: Vec<SyntaxNode> = params
                    .into_inner()
                    .map(|param| {
                        self.walk_tree(param, visitor)
                            .unwrap_or_else(|err| Some(SyntaxNode::create_error(err)))
                            .unwrap_or_else(|| {
                                SyntaxNode::create_error("No expression was created")
                            })
                    })
                    .collect();
                let func_call_node = visitor.visit_func_call(&func_name, params)?;
                Ok(Some(func_call_node))
            }

            Rule::number => {
                let parse_node = parse_node.as_str().to_string();
                let res = visitor
                    .visit_number(parse_node)
                    .unwrap_or_else(SyntaxNode::from);
                Ok(Some(res))
            }
            Rule::linebreak => {
                self.current_line += 1;
                Ok(None)
            }
            Rule::COMMENT | Rule::WHITESPACE | Rule::EOI => Ok(None),
            _ => {
                log::warn!(
                    "Unimplemented rule `{:?}`:\n{}. {}",
                    parse_node.as_rule(),
                    self.current_line,
                    parse_node.as_str()
                );
                Ok(None)
            }
        }
    }

    fn get_function_parts<'a>(
        &self,
        func: ParseNode<'a>,
    ) -> Result<FunctionParts<'a>, SyntaxBuilderError> {
        let mut func_components = func.into_inner();
        let type_specifier = func_components
            .next()
            .ok_or("Missing function return type specifier")?
            .as_str()
            .into();
        let ident = func_components
            .next()
            .ok_or("Missing function identifier")?
            .as_str()
            .into();
        let params = func_components.next().ok_or("Missing function params")?;
        let func_body = func_components.next().ok_or("Missing function body")?;
        Ok(FunctionParts {
            return_type: type_specifier,
            name: ident,
            params,
            body: func_body,
        })
    }

    fn handle_param_decl(
        &mut self,
        params: ParseNode,
        visitor: &mut Visitor,
    ) -> Result<(), SyntaxBuilderError> {
        for pair in params.into_inner() {
            if ReturnType::from(pair.as_str()) == ReturnType::Void {
                break;
            }
            let (type_spec, ident) = pair
                .into_inner()
                .collect_tuple()
                .expect("Invariant violated: No pair of type specifier & identifier");
            let type_spec = ReturnType::from(type_spec.as_str());
            let ident = SymbolName::from(ident.as_str());
            log::trace!("PARAM: {} {}", type_spec, ident);
            visitor.visit_param_decl(ident, type_spec, self.current_line)?;
        }
        Ok(())
    }
}
