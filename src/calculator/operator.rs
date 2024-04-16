use std::{fmt::Display, str::FromStr};

pub use crate::calculator::error::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponential,
}
impl Operator {
    pub fn as_char(&self) -> char {
        return match self {
            Operator::Add => '+',
            Operator::Subtract => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
            Operator::Modulo => '%',
            Operator::Exponential => '^',
        };
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_char());
    }
}
impl TryFrom<char> for Operator {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            '%' => Ok(Operator::Modulo),
            '^' => Ok(Operator::Exponential),
            _ => Err(ParseOperatorError::InvalidCharacter)?,
        };
    }
}
impl FromStr for Operator {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseOperatorError::EmptyString)?;
        }

        if s.len() > 1 {
            Err(ParseOperatorError::TooLong)?;
        }

        return s.chars().next().expect("s is not empty").try_into();
    }
}
