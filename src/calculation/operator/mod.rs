use std::convert::TryFrom;

use super::Calculation;
use crate::error::BadOperatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Times,
    Div,
}

impl Operator {
    pub fn apply(&self, lhs: &Calculation, rhs: &Calculation) -> isize {
        let left = lhs.evaluate();
        let right = rhs.evaluate();
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Times => left * right,
            Operator::Div => left / right,
        }
    }

    pub fn rank(&self) -> usize {
        match self {
            Operator::Add | Operator::Sub => 1,
            Operator::Times | Operator::Div => 2,
        }
    }
}
impl TryFrom<char> for Operator {
    type Error = BadOperatorError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let variant = match value {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Times,
            '/' => Self::Div,
            _ => Err(Self::Error::new(value.to_string()))?,
        };
        Ok(variant)
    }
}
