use pest::{error::Error as ParseError, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LexicalParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, ParseError<Rule>> {
    let pairs = LexicalParser::parse(Rule::program, input)?;
    Ok(pairs)
}

pub fn test() {}
