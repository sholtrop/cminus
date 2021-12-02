use pest::{
    error::Error as ParseError,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
// lazy_static! {
//     static ref PREC_CLIMBER: PrecClimber<Rule> = {
//         use Assoc::*;
//         use Rule::*;

//         PrecClimber::new(vec![
// Operator::new(assign, Right),
// Operator::new(or, Left) | Operator::new(and, Left),
// Operator::new(gt, Left)
//     | Operator::new(gte, Left)
//     | Operator::new(lt, Left)
//     | Operator::new(lte, Left),
// Operator::new(eq, Left) | Operator::new(neq, Left),
// Operator::new(add, Left) | Operator::new(sub, Left),
// Operator::new(mul, Left) | Operator::new(div, Left) | Operator::new(modulo, Left),
//         ])
//     };
// }

pub type ParseTree<'a> = Pairs<'a, Rule>;
pub type ParseNode<'a> = Pair<'a, Rule>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LexicalParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, ParseError<Rule>> {
    let pairs = LexicalParser::parse(Rule::program, input)?;
    Ok(pairs)
}
