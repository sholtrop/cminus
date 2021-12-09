use pest::{
    error::Error as ParseError,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

pub type ParseTree<'a> = Pairs<'a, Rule>;
pub type ParseNode<'a> = Pair<'a, Rule>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LexicalParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, ParseError<Rule>> {
    let pairs = LexicalParser::parse(Rule::program, input)?;
    Ok(pairs)
}
