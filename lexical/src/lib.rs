use pest::{error::Error as ParseError, Parser};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LexicalParser;

pub fn parse() -> Result<(), ParseError<Rule>> {
    let pairs = LexicalParser::parse(
        Rule::program,
        "
    int main(void) {
        int a = 0, b;
        return 0;
    }",
    )
    .map_err(|err| {
        log::error!("{}", err);
        err
    })?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        // println!("Miep:    {:?}", pair.tokens());
        println!();
    }

    Ok(())
}
