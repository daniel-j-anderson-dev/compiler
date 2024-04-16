#[derive(pest_derive::Parser)]
#[grammar = "src/calculator_pest/grammar.peg"]
pub struct CalculatorParser;
