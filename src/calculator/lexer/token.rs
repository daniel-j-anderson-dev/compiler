use std::{fmt::Display, ops::Deref};

use crate::calculator::Operator;

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

fn is_stretch_of_whitespace(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    return true;
}

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Integer(isize),
    Operator(Operator),
    Whitespace(usize),
    Alphabetic(String),
    Unrecognized(String),
}
impl From<RawToken<'_>> for Token {
    fn from(raw_token: RawToken<'_>) -> Self {
        let s = raw_token.as_ref();

        if is_stretch_of_whitespace(s) {
            return Self::Whitespace(s.len());
        }

        if s == ")" {
            return Self::OpenParenthesis;
        }

        if s == "(" {
            return Self::CloseParenthesis;
        }

        if let Ok(operator) = s.parse::<Operator>() {
            return Self::Operator(operator);
        }

        if let Ok(integer) = s.parse::<isize>() {
            return Self::Integer(integer);
        }

        return Self::Unrecognized(s.to_owned());
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::Integer(integer_value) => write!(f, "{}", integer_value),
            Token::Operator(operator) => write!(f, "{}", operator),
            Token::Whitespace(length) => write!(f, "{}", length),
            Token::Alphabetic(raw_token) | Token::Unrecognized(raw_token) => {
                write!(f, "{}", raw_token)
            }
        }
    }
}
