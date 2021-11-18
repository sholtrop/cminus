use lexical::{ParseTree, Rule};

use crate::{error::SyntaxBuilderError, id::SymbolName, symbol::ReturnType, visitor::Visitor};

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
    ) -> Result<(), SyntaxBuilderError> {
        for node in parse_tree {
            match node.as_rule() {
                Rule::fn_declaration => {
                    let mut func_components = node.into_inner();
                    let type_specifier = func_components
                        .next()
                        .ok_or("Missing function return type specifier")?
                        .as_str();
                    let ident = func_components
                        .next()
                        .ok_or("Missing function identifier")?
                        .as_str()
                        .into();
                    let params = func_components.next().ok_or("Missing function params")?;
                    let func_body = func_components.next().ok_or("Missing function body")?;
                    log::trace!(
                        "Function decl:\nret: {}\nname: {}\nparams: {}\n",
                        type_specifier,
                        ident,
                        params.as_str(),
                    );
                    visitor
                        .visit_func_start(SymbolName(ident), ReturnType::from(type_specifier))?;

                    self.walk_tree(func_body.into_inner(), visitor)?;
                    visitor.visit_func_end();
                }
                Rule::statement => {
                    log::trace!("statement `{}`", node.as_str());
                }

                Rule::COMMENT | Rule::WHITESPACE | Rule::EOI => {}
                Rule::linebreak => {
                    self.current_line += 1;
                }
                _ => {
                    log::warn!(
                        "Unimplemented rule `{:?}`: {}. {}",
                        node.as_rule(),
                        self.current_line,
                        node.as_str()
                    );
                }
            }
        }
        Ok(())
    }
}
