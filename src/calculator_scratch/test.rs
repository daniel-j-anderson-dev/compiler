#![cfg(test)]

use super::*;

#[test]
fn test_lexer() {
    let source_code = "(1   +  2)-3";
    let mut lexer = Lexer::new(source_code);
    for i in 0..10 {
        dbg!(lexer.next());
    }
}