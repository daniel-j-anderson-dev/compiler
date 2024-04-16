use std::{fmt::Display, str::FromStr};

use crate::calculator_pest::ParseOperatorError;

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
    fn as_char(&self) -> char {
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
impl TryFrom<char> for Operator {
    type Error = ParseOperatorError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            '%' => Ok(Operator::Modulo),
            '^' => Ok(Operator::Exponential),
            _ => Err(ParseOperatorError::InvalidCharacter),
        };
    }
}
impl FromStr for Operator {
    type Err = ParseOperatorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "%" => Ok(Operator::Modulo),
            "^" => Ok(Operator::Exponential),
            _ => Err(ParseOperatorError::InvalidCharacter),
        };
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_char());
    }
}
