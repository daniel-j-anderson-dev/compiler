pub mod node;

use crate::{CalculatorParser, ParseNodeError, Rule};
pub use node::{Node, Operator};

use std::{ops::Deref, str::FromStr};

use pest::Parser;

pub struct AbstractSyntaxTree(Vec<Node>);
impl AsRef<[Node]> for AbstractSyntaxTree {
    fn as_ref(&self) -> &[Node] {
        return self.0.as_ref();
    }
}
impl Deref for AbstractSyntaxTree {
    type Target = [Node];
    fn deref(&self) -> &Self::Target {
        return self.as_ref();
    }
}
impl FromStr for AbstractSyntaxTree {
    type Err = ParseNodeError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut abstract_syntax_tree = Vec::<Node>::new();
        for pair in CalculatorParser::parse(Rule::Program, source)? {
            dbg!(&pair);
            match pair.as_rule() {
                Rule::Expression => abstract_syntax_tree.push(pair.try_into()?),
                _ => {}
            }
        }
        return Ok(Self(abstract_syntax_tree));
    }
}
