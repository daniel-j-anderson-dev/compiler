pub mod token;
pub use token::{RawToken, Token};

use super::Operator;

pub trait Tokenize<'a> {
    fn tokenize(&'a self) -> Lexer<'a>;
}
impl<'a, T: AsRef<str>> Tokenize<'a> for T {
    fn tokenize(&'a self) -> Lexer<'a> {
        return Lexer::new(self.as_ref());
    }
}

pub struct Lexer<'a> {
    content: &'a str,
}
impl<'a> Lexer<'a> {
    pub const fn new(source_code: &'a str) -> Self {
        return Self {
            content: source_code,
        };
    }

    fn cut(&mut self, index: usize) -> RawToken<'a> {
        let removed = RawToken(&self.content[..index]);
        self.content = &self.content[index..];
        return removed;
    }

    fn cut_uniform_token(&mut self, predicate: impl Fn(char) -> bool) -> RawToken<'a> {
        let token_end = self.find_uniform_token_end(predicate);
        let removed = self.cut(token_end);
        return removed;
    }

    fn find_uniform_token_end(&self, predicate: impl Fn(char) -> bool) -> usize {
        let mut token_end = 0;
        for (character_index, character) in self.content.char_indices() {
            if !predicate(character) {
                break;
            }
            token_end = character_index + 1;
        }
        return token_end;
    }

    fn first_char(&self) -> Option<char> {
        return self.content.chars().next();
    }

    fn is_whitespace(c: char) -> bool {
        return c.is_whitespace() || c == '\0';
    }

    fn is_unrecognized(c: char) -> bool {
        return !c.is_alphanumeric() || c.is_whitespace() || c == '(' || c == ')';
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let first_character = self.first_char()?;

        let token = if first_character.is_whitespace() {
            let raw_token = self.cut_uniform_token(Self::is_whitespace);
            Token::Whitespace(raw_token.len())
        } else if first_character.is_numeric() {
            let raw_token = self.cut_uniform_token(char::is_numeric);
            Token::Integer(raw_token.parse().expect("only numeric chars"))
        } else if let Ok(operator) = Operator::try_from(first_character) {
            let _raw_token = self.cut(1);
            Token::Operator(operator)
        } else if first_character.is_alphabetic() {
            let raw_token = self.cut_uniform_token(char::is_alphabetic);
            Token::Alphabetic(raw_token.to_owned())
        } else if first_character == '(' {
            let _raw_token = self.cut(1);
            Token::OpenParenthesis
        } else if first_character == ')' {
            let _raw_token = self.cut(1);
            Token::CloseParenthesis
        } else {
            let raw_token = self.cut_uniform_token(Self::is_unrecognized);
            Token::Unrecognized(raw_token.to_owned())
        };

        return Some(token);
    }
}
