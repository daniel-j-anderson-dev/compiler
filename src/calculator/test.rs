#![cfg(test)]

use macros::*;

use super::*;

#[test]
fn lexer() {
    const SOURCE_CODE: &str = "   abc123+-*/%^()!@#$😀&";
    let mut lexer = Lexer::new(SOURCE_CODE);
    
    assert_eq!(lexer.next(), Some(Token::Whitespace(3)));
    assert_eq!(lexer.next(), Some(Token::Alphabetic("abc".to_owned())));
    assert_eq!(lexer.next(), Some(Token::Integer(123)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Add)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Subtract)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Multiply)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Divide)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Modulo)));
    assert_eq!(lexer.next(), Some(Token::Operator(Operator::Exponential)));
    assert_eq!(lexer.next(), Some(Token::OpenParenthesis));
    assert_eq!(lexer.next(), Some(Token::CloseParenthesis));
    assert_eq!(lexer.next(), Some(Token::Unrecognized("!@#$😀&".to_owned())));
}

#[test]
fn lexer_whitespace() {
    const SOURCE_CODE: &str = "\n \0 \n";
    assert_eq!(Lexer::new(SOURCE_CODE).next(), Some(Token::Whitespace(5)));
}

#[test]
fn macro_example() {
    generate_4_fn!();
    assert_eq!(generated_4_fn(), 4);
}
