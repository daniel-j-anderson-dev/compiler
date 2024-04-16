pub mod token;

pub struct Lexer<'a> {
    source_code: &'a str,
    index: usize,
}
impl<'a> Lexer<'a> {
    fn new(source_code: &'a str) -> Self {
        return Self { 
            source_code,
            index: 0,
        };
    }
}
