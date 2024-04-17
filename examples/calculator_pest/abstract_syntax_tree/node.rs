pub mod operator;
pub use operator::Operator;

use std::fmt::Display;

use crate::{error::ParseNodeError, grammar::Rule, InvalidSignChar};

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Debug, PartialEq)]
pub enum Node {
    Integer(isize),
    UnaryExpression {
        operator: Operator,
        operand: Box<Node>,
    },
    BinaryExpression {
        operator: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
impl Node {
    pub fn new_unary_expression(operator: Pair<'_>, operand: Node) -> Result<Self, ParseNodeError> {
        return Ok(Node::UnaryExpression {
            operator: operator.as_str().parse()?,
            operand: Box::new(operand),
        });
    }
    pub fn new_binary_expression(
        operator: Pair<'_>,
        lhs: Node,
        rhs: Node,
    ) -> Result<Self, ParseNodeError> {
        return Ok(Node::BinaryExpression {
            operator: operator.as_str().parse()?,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
    }
}
impl TryFrom<Option<Pair<'_>>> for Node {
    type Error = ParseNodeError;
    fn try_from(value: Option<Pair<'_>>) -> Result<Self, Self::Error> {
        match value {
            Some(pair) => pair.try_into(),
            None => Err(ParseNodeError::NoNextPair),
        }
    }
}
impl TryFrom<Pair<'_>> for Node {
    type Error = ParseNodeError;
    fn try_from(pair: Pair<'_>) -> Result<Self, Self::Error> {
        let rule = pair.as_rule();
        let mut pair = pair.into_inner();

        return match rule {
            Rule::Expression => Node::try_from(pair.next()),
            Rule::UnaryExpression => {
                let operator = pair.next().ok_or(ParseNodeError::NoNextPair)?;
                let operand = Node::try_from(pair.next())?;
                Node::new_unary_expression(operator, operand)
            }
            Rule::BinaryExpression => {
                let mut lhs = Node::try_from(pair.next())?;
                let op = pair.next().ok_or(ParseNodeError::NoNextPair)?;
                let mut rhs = Node::try_from(pair.next())?;

                let mut ret_val = Node::new_binary_expression(op, lhs, rhs)?;
                loop {
                    if let Some(operator) = pair.next() {
                        lhs = ret_val;
                        rhs = pair.next().try_into()?;
                        ret_val = Node::new_binary_expression(operator, lhs, rhs)?;
                    } else {
                        return Ok(ret_val);
                    }
                }
            }
            Rule::Integer => {
                let raw_value = pair.as_str();
                let sign = match raw_value.chars().next() {
                    Some('+') => 1,
                    Some('-') => -1,
                    _ => Err(InvalidSignChar)?,
                };
                let value = raw_value[1..].parse::<isize>()?;
                Ok(Node::Integer(sign * value))
            }
            unknown => Err(ParseNodeError::UnrecognizedRule(unknown)),
        };
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Node::Integer(value) => write!(f, "{}", value),
            Node::UnaryExpression { operator, operand } => write!(f, "{}{}", operator, operand),
            Node::BinaryExpression { operator, lhs, rhs } => {
                write!(f, "{} {} {}", lhs, operator, rhs)
            }
        };
    }
}
