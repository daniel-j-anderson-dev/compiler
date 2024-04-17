#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basics() {
        assert!("b".parse::<AbstractSyntaxTree>().is_err());
    }

    #[test]
    fn unary_expr() {
        let plus_one = "+1".parse::<AbstractSyntaxTree>().unwrap();
        assert_eq!(
            plus_one.as_ref(),
            &[Node::UnaryExpression {
                operator: Operator::Add,
                operand: Box::new(Node::Integer(1))
            }]
        );
        assert_eq!(format!("{}", plus_one[0]), "+1");

        let neg_two = "-2".parse::<AbstractSyntaxTree>().unwrap();
        assert_eq!(
            neg_two.as_ref(),
            &[Node::UnaryExpression {
                operator: Operator::Subtract,
                operand: Box::new(Node::Integer(2))
            }]
        );
        assert_eq!(neg_two[0].to_string(), "-2");
    }
    #[test]
    fn binary_expr() {
        let sum = "1 + 2".parse::<AbstractSyntaxTree>().unwrap();
        assert_eq!(
            sum.as_ref(),
            &[Node::BinaryExpression {
                operator: Operator::Add,
                lhs: Box::new(Node::Integer(1)),
                rhs: Box::new(Node::Integer(2))
            }]
        );
        assert_eq!(sum[0].to_string(), "1 + 2");
        let subtract = "1   -  \t  2".parse::<AbstractSyntaxTree>().unwrap();
        assert_eq!(
            subtract.as_ref(),
            &[Node::BinaryExpression {
                operator: Operator::Subtract,
                lhs: Box::new(Node::Integer(1)),
                rhs: Box::new(Node::Integer(2))
            }]
        );
        assert_eq!(subtract[0].to_string(), "1 - 2");
        // fails as there's no rhs:
        // let paran_sum = parse("(1 + 2)");
        // assert!(paran_sum.is_ok());
    }

    #[test]
    fn nested_expr() {
        fn test_expr(expected: &str, src: &str) {
            assert_eq!(
                expected,
                src.parse::<AbstractSyntaxTree>()
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
            );
        }

        test_expr("1 + 2 + 3", "(1 + 2) + 3");
        test_expr("1 + 2 + 3", "1 + (2 + 3)");
        test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
        test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(
            "1+2+3".parse::<AbstractSyntaxTree>().unwrap().as_ref(),
            &[Node::BinaryExpression {
                operator: Operator::Add,
                lhs: Box::new(Node::BinaryExpression {
                    operator: Operator::Add,
                    lhs: Box::new(Node::Integer(1)),
                    rhs: Box::new(Node::Integer(2)),
                }),
                rhs: Box::new(Node::Integer(3)),
            }]
        )
    }
}
