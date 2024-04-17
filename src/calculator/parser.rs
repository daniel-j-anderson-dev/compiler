pub mod abstract_syntax_tree;
pub use abstract_syntax_tree::AbstractSyntaxTree;

use crate::calculator::{
    lexer::{Token, Tokenize},
    error::*
};

pub struct Parser {
    tokens: Vec<Token>,
}
impl Parser {
    pub fn new(source_code: &str) -> Self {
        return source_code.tokenize().collect();
    }
    pub fn parse(self) -> Result<AbstractSyntaxTree, ParseError> {
        let mut nodes = Vec::new();

        

        return Ok(AbstractSyntaxTree::new(nodes));
    }
}
impl FromIterator<Token> for Parser {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        return Self {
            tokens: iter.into_iter().collect::<Vec<_>>(),
        }
    }
}
