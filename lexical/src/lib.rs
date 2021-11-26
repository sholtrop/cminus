use pest::{
    error::Error as ParseError,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use pest_derive::Parser;

use lazy_static::lazy_static;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(assign, Right),
            Operator::new(or, Left) | Operator::new(and, Left),
            Operator::new(gt, Left)
                | Operator::new(gte, Left)
                | Operator::new(lt, Left)
                | Operator::new(lte, Left),
            Operator::new(eq, Left) | Operator::new(neq, Left),
            Operator::new(add, Left) | Operator::new(sub, Left),
            Operator::new(mul, Left) | Operator::new(div, Left) | Operator::new(modulo, Left),
        ])
    };
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LexicalParser;

fn consume(_: ParseNode) {}

pub fn parse(input: &str) -> Result<Pairs<Rule>, ParseError<Rule>> {
    use Rule::*;
    let pairs = LexicalParser::parse(Rule::program, input)?;
    PREC_CLIMBER.climb(pairs, consume, |lhs, op, rhs| match op.as_rule() {
        add => {}
        _ => unreachable!(),
    });

    Ok(pairs)
}

pub type ParseTree<'a> = Pairs<'a, Rule>;
pub type ParseNode<'a> = Pair<'a, Rule>;
