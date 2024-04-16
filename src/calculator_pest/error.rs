use std::fmt::{Debug, Display};

use thiserror::Error;

use crate::calculator_pest::grammar::Rule;

#[derive(Debug, Error)]
pub enum ParseNodeError {
    #[error("{0}")]
    Pest(
        #[from]
        #[source]
        pest::error::Error<Rule>,
    ),

    #[error("No next pair")]
    NoNextPair,

    #[error("{0}")]
    ParseOperator(
        #[from]
        #[source]
        ParseOperatorError,
    ),

    #[error("{0}")]
    ParseIntegerError(
        #[source]
        Either<std::num::ParseIntError, InvalidSignChar>,
    ),

    #[error("Unimplemented Rule: {0:?}")]
    UnrecognizedRule(Rule),
}
impl From<std::num::ParseIntError> for ParseNodeError {
    fn from(value: std::num::ParseIntError) -> Self {
        return Self::ParseIntegerError(Either::Left(value));
    }
}
impl From<InvalidSignChar> for ParseNodeError {
    fn from(value: InvalidSignChar) -> Self {
        return Self::ParseIntegerError(Either::Right(value));
    }
}


#[derive(Debug, Error)]
#[error("invalid sign char in Integer")]
pub struct InvalidSignChar;

#[derive(Error)]
pub enum Either<L, R> {
    Left(
        #[source]
        #[from]
        L
    ),
    Right(
        #[source]
        R
    ),
}
impl<L: Debug, R: Debug> Debug for Either<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Either::Left(value) => value.fmt(f),
            Either::Right(value) => value.fmt(f),
        };
    }
}
impl<L: Display, R: Display> Display for Either<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Either::Left(value) => value.fmt(f),
            Either::Right(value) => value.fmt(f),
        };
    }
}
impl<L, R> Either<L, R> {
    pub fn left(value: L) -> Self {
        return Self::Left(value)
    }
    pub fn right(value: R) -> Self {
        return Self::Right(value)
    }
}

#[derive(Debug, Error)]
pub enum ParseOperatorError {
    #[error("Invalid character")]
    InvalidCharacter,
}
