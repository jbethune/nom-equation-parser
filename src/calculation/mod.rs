pub mod operator;

use std::str::FromStr;

use crate::error::InvalidFormulaError;
use crate::parse::parse_equation;
use operator::Operator;

pub type ErrorPosition = usize;

#[derive(Debug)]
pub enum Calculation {
    Literal(isize),
    Unevaluated {
        lhs: Box<Calculation>,
        operator: Operator,
        rhs: Box<Calculation>,
    },
}

impl Calculation {
    pub fn evaluate(&self) -> isize {
        match self {
            Self::Literal(n) => *n,
            Self::Unevaluated { lhs, operator, rhs } => operator.apply(lhs, rhs),
        }
    }
}

impl FromStr for Calculation {
    type Err = InvalidFormulaError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (tail, result) = parse_equation(input)?;
        if !tail.is_empty() {
            Err(Self::Err {
                failed_tail: tail.to_string(),
            })
        } else {
            Ok(result)
        }
    }
}
