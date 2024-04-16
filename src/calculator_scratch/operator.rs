pub use crate::calculator_scratch::error::*;

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponential,
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