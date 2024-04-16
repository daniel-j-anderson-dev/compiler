use crate::calculator_scratch::error::ParseError;

use std::str::FromStr;

pub enum TokenKind {
    Integer,
    Operator,
    UnaryExpression,
    BinaryExpression,
    Whitespace,
}

pub struct RawToken<'a> {
    value: &'a str,
    kind: TokenKind,
}
impl<'a> FromStr for RawToken<'a> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.char_indices();
        todo!();
    }
}