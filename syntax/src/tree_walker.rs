use crate::{
    error::SyntaxBuilderError, id::SymbolName, node::SyntaxNode, symbol::ReturnType,
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

    pub fn walk_tree(
        &mut self,
        parse_tree: ParseTree,
        visitor: &mut Visitor,
    ) -> Result<SyntaxNode, SyntaxBuilderError> {
        for node in parse_tree {
            match node.as_rule() {
                Rule::fn_declaration => {
                    let FunctionParts {
                        return_type,
                        name,
                        params,
                        body,
                    } = self.get_function_parts(node)?;
                    log::trace!(
                        "Function decl:\nret: {}\nname: {}\nparams: {}\n",
                        return_type,
                        name,
                        params.as_str(),
                    );
                    visitor.visit_func_start(
                        SymbolName(name),
                        ReturnType::from(return_type.as_str()),
                    )?;
                    self.handle_param_decl(params, visitor)?;
                    let func_body = self.walk_tree(body.into_inner(), visitor)?;
                    // visitor.att
                    // visitor.visit_func_end();
                }
                Rule::compound_stmt => {
                    log::trace!("compound statement");
                    visitor.add_local_scope();
                    self.walk_tree(node.into_inner(), visitor)?;
                    visitor.leave_local_scope();
                }
                // Rule::statement => visitor.visit_statement(node),
                Rule::linebreak => {
                    self.current_line += 1;
                }
                Rule::COMMENT | Rule::WHITESPACE | Rule::EOI => {}
                _ => {
                    log::warn!(
                        "Unimplemented rule `{:?}`:\n{}. {}",
                        node.as_rule(),
                        self.current_line,
                        node.as_str()
                    );
                }
            }
        }
        Ok(SyntaxNode::Empty)
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
