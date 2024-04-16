pub mod token;
pub use crate::calculator_scratch::{operator::Operator, lexer::token::{RawToken, TokenKind}};


pub struct Lexer<'a> {
    content: &'a str,
}
impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        return Self { 
            content: source_code,
        };
    }

    fn cut(&mut self, index: usize) -> RawToken<'a> {
        let removed = RawToken(&self.content[..index]);
        self.content = &self.content[index..];
        return removed;
    }
    
    fn cut_uniform_token(&mut self, token_definition: impl Fn(char) -> bool) -> RawToken<'a> {
        let token_end = self.find_uniform_token_end(token_definition);
        let removed = self.cut(token_end);
        return removed;
    }

    fn find_uniform_token_end(&self, token_definition: impl Fn(char) -> bool) -> usize {
        let mut chars = self.content.char_indices();
        let mut token_end = 0;
        while let Some((index, character)) = chars.next() {
            token_end = index;
            
            if !token_definition(character) {
                break;
            }
        }
        return token_end;
    }

    fn first_char(&self) -> Option<char> {
        return self.content.chars().next();
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = RawToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let first_character = self.first_char()?;
        
        return if first_character.is_whitespace() {
            self.cut_uniform_token(char::is_whitespace);
            self.next()
        }
        else if first_character.is_numeric() {
            Some(self.cut_uniform_token(char::is_numeric))
        }
        else if first_character.is_alphabetic() {
            Some(self.cut_uniform_token(char::is_alphanumeric))
        }
        else {
            Some(self.cut(1))
        };
    }
}
