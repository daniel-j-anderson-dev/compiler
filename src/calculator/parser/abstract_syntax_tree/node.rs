use crate::calculator::Operator;

pub enum Node {
    Expression(Expression),
    Operator(Operator),
}

pub enum Term {
    Integer(isize),
    Expression(Box<Expression>),
}

pub enum Expression {
    UnaryExpression {
        operator: Operator,
        operand: Box<Term>,
    },
    BinaryExpression {
        operator: Operator,
        lhs: Box<Term>,
        rhs: Box<Term>,
    },
    Term(Term),
}
