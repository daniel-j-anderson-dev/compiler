pub mod node;
use std::str::FromStr;

use node::Node;

use crate::calculator::{ParseError, Tokenize};

use super::Parser;

pub struct AbstractSyntaxTree(Vec<Node>);
impl AbstractSyntaxTree {
    pub fn new(value: Vec<Node>) -> Self {
        return Self(value);
    }
}
impl FromStr for AbstractSyntaxTree {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return s.tokenize().collect::<Parser>().parse();
    }
}
