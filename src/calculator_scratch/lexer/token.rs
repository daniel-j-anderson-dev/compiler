use crate::calculator_scratch::error::ParseError;

use std::{ops::{Deref, DerefMut}, str::FromStr};

pub enum TokenKind {
    Integer,
    Operator,
    UnaryExpression,
    BinaryExpression,
    Whitespace,
}

#[derive(Debug)]
pub struct RawToken<'a>(pub &'a str);
impl<'a> Deref for RawToken<'a> {
    type Target = &'a str;
    fn deref(&self) -> &&'a str {
        return &self.0;
    }
}
impl<'a> AsRef<str> for RawToken<'a> {
    fn as_ref(&self) -> &'a str {
        return self.0;
    }
}