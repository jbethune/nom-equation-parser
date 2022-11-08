mod tree;

use nom::branch::alt;
use nom::character::complete::{char as character, digit1, multispace0};
use nom::combinator::{map, map_res, opt, peek};
use nom::sequence::tuple;
use nom::IResult;

use crate::calculation::operator::Operator;
use crate::calculation::Calculation;
use tree::combine_operands_and_operators;

fn parse_numeric_literal(input: &str) -> IResult<&str, isize> {
    // TODO swap order of opt() and alt() as an excercise
    let mut parser = tuple((
        opt(alt((character('+'), character('-')))), // the sign on the number, if any
        digit1,                                     // one or more digits
    ));
    // first ensure that we have a number and if so, split it off
    let (tail, (sign, number_text)) = parser(input)?;
    // then convert that recognized fragment by using Rust's stdlib parse to isize
    let value: isize = number_text.parse().expect("already validated");
    let result = match sign {
        Some('+') | None => value,
        _ => -value,
    };
    Ok((tail, result))
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let parser = alt((
        character('+'),
        character('-'),
        character('*'),
        character('/'),
    ));
    map_res(parser, |c| c.try_into())(input)
}

fn parse_parentheses(input: &str) -> IResult<&str, Calculation> {
    let (tail, _) = character('(')(input)?;
    let (tail, calculation) = parse_equation(tail)?;
    let (tail, _) = character(')')(tail)?;
    Ok((tail, calculation))
}

/// Parse a single operand
///
/// An operand is either a single numeric literal or a more complex expression in parentheses,
/// which for our purposes is still treated like a single entity.
fn parse_operand(input: &str) -> IResult<&str, Calculation> {
    alt((
        map(parse_numeric_literal, Calculation::Literal),
        parse_parentheses,
    ))(input)
}

pub fn parse_equation(input: &str) -> IResult<&str, Calculation> {
    let closing_paren = character::<&str, nom::error::Error<&str>>(')');
    let mut rest = input;

    let mut operands: Vec<Calculation> = vec![];
    let mut operators: Vec<Operator> = vec![];

    loop {
        let (tail, _) = multispace0(rest)?; // always succeeds
        let (tail, operand) = parse_operand(tail)?;
        operands.push(operand);

        let (tail, _) = multispace0(tail)?;

        // two checks to see if we are at the end of the equation
        if tail.is_empty() || peek(&closing_paren)(tail).is_ok() {
            rest = tail;
            break;
        }
        // else: we have to look further right

        let (tail, operator) = parse_operator(tail)?;
        let (tail, _) = multispace0(tail)?;
        operators.push(operator);
        rest = tail;
    }

    // we have gathered all n operatorsand and all n+1 operands on the same level.
    // Parsing is over. Now we need to construct the equation tree
    assert_eq!(operands.len(), operators.len() + 1);

    let result = combine_operands_and_operators(operands, operators);
    Ok((rest, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_parsing() {
        let fun = parse_numeric_literal;
        assert_eq!(fun("3"), Ok(("", 3)));
        assert_eq!(fun("42"), Ok(("", 42)));
        assert_eq!(fun("42 11"), Ok((" 11", 42)));
        assert_eq!(fun("+42"), Ok(("", 42)));
        assert_eq!(fun("-42"), Ok(("", -42)));
        assert_eq!(fun("+42-3"), Ok(("-3", 42)));
        assert_eq!(fun("-42-3"), Ok(("-3", -42)));
        assert_eq!(fun("42-3"), Ok(("-3", 42)));
    }

    #[test]
    fn test_parse_operator() {
        let fun = parse_operator;
        assert_eq!(fun("+"), Ok(("", Operator::Add)));
        assert_eq!(fun("-"), Ok(("", Operator::Sub)));
        assert_eq!(fun("*"), Ok(("", Operator::Times)));
        assert_eq!(fun("/"), Ok(("", Operator::Div)));
        assert_eq!(fun("+ foo"), Ok((" foo", Operator::Add)));
    }

    #[test]
    fn test_parse_equation() {
        fn fun(expr: &str) -> isize {
            parse_equation(expr).expect("test").1.evaluate()
        }

        assert_eq!(fun("4"), 4);
        assert_eq!(fun("4 + 3"), 7);
        assert_eq!(fun("4 - 1)"), 3);
        assert_eq!(fun("4 - (3 + 2)"), -1);
        assert_eq!(fun("2 * 3 + 4"), 10);
        assert_eq!(fun("(2 * 3) + 4"), 10);
        assert_eq!(fun("3 * 5 + 2 * 7"), 29);
        assert_eq!(fun("-4 * -2"), 8);
        assert_eq!(fun("100 - 10 - 10 - 10 - 10 - 10"), 50);
        assert_eq!(fun("1024 / 2 / 2 / 2 / 2 / 2"), 32);
        assert_eq!(fun("2 * (3 * (4 * (5 + 7) + 8) + 9) + 10"), 364);
        assert_eq!(fun("-3 - -4"), 1);
        assert_eq!(fun("2 + 3 * 4 * 5 + 6"), 68);
        assert_eq!(fun("5 - (-5 ) * 2"), 15);

        // TODO add more unit tests just like this
        assert_eq!(
            fun("2 * 3 + 4 - (3 - 6 * 2) * (4 + 3)"),
            /**/ 2 * 3 + 4 - (3 - 6 * 2) * (4 + 3)
        );
    }
}
