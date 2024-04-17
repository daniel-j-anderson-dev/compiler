pub mod abstract_syntax_tree;
pub use abstract_syntax_tree::*;

use crate::calculator::{
    Token, Tokenize,
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
        let mut tokens = self.tokens.into_iter().enumerate().peekable();

        let mut open_paren_indices = Vec::<usize>::new();

        let mut nodes = Vec::new();

        while let Some((index, token)) = tokens.next() {
            match token {
                Token::OpenParenthesis => open_paren_indices.push(index),
                Token::CloseParenthesis => {
                    let open_paren_index = open_paren_indices.pop().ok_or(ParseError::MissingOpenParenthesis)?;
                },
                Token::Integer(_) => todo!(),
                Token::Operator(_) => todo!(),
                Token::Alphabetic(_) |
                Token::Whitespace(_) |
                Token::Unrecognized(_) => continue,
            }
        }

        if !open_paren_indices.is_empty() {
            return Err(ParseError::MissingCloseParenthesis);
        }

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
