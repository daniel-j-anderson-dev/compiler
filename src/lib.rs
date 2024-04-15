use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "calculator.peg"]
pub struct CalculatorParser;
