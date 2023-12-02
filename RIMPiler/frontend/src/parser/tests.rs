use crate::parser::{parse_arithmetic_expression, parse_boolean_expression, parse_relations};
use crate::AST::{
    ArithmeticExpression, ArithmeticOperator, Assignment, BooleanExpression, BooleanOperator,
    Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator,
};

#[test]
fn basic_parse() {
    let mut tokeniser = crate::lexer::Tokeniser::new().initialise();

    let tokens = tokeniser.tokenise("9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(result.unwrap(), ArithmeticExpression::Integer(9));

    let tokens = tokeniser.tokenise("1 + 2".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Addition,
            Box::new(ArithmeticExpression::Integer(1)),
            Box::new(ArithmeticExpression::Integer(2)),
        )
    );

    let tokens = tokeniser.tokenise("1 + 2 * 3".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Addition,
            Box::new(ArithmeticExpression::Integer(1)),
            Box::new(ArithmeticExpression::Operation(
                ArithmeticOperator::Multiplication,
                Box::new(ArithmeticExpression::Integer(2)),
                Box::new(ArithmeticExpression::Integer(3)),
            )),
        )
    );

    let tokens = tokeniser.tokenise("1 + 2 ^ 5 + 8 * 3".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Addition,
            Box::new(ArithmeticExpression::Operation(
                ArithmeticOperator::Addition,
                Box::new(ArithmeticExpression::Integer(1)),
                Box::new(ArithmeticExpression::Operation(
                    ArithmeticOperator::Exponentiation,
                    Box::new(ArithmeticExpression::Integer(2)),
                    Box::new(ArithmeticExpression::Integer(5)),
                )),
            )),
            Box::new(ArithmeticExpression::Operation(
                ArithmeticOperator::Multiplication,
                Box::new(ArithmeticExpression::Integer(8)),
                Box::new(ArithmeticExpression::Integer(3)),
            )),
        )
    );

    let tokens = tokeniser.tokenise("-1".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Unary(
            UnaryArithmeticOperator::Negation,
            Box::new(ArithmeticExpression::Integer(1))
        )
    );

    let tokens = tokeniser.tokenise("6 * -7^9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Multiplication,
            Box::new(ArithmeticExpression::Integer(6)),
            Box::new(ArithmeticExpression::Unary(
                UnaryArithmeticOperator::Negation,
                Box::new(ArithmeticExpression::Operation(
                    ArithmeticOperator::Exponentiation,
                    Box::new(ArithmeticExpression::Integer(7)),
                    Box::new(ArithmeticExpression::Integer(9)),
                )),
            )),
        )
    );

    let tokens = tokeniser.tokenise("(6 * -7) + 9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Addition,
            Box::new(ArithmeticExpression::Operation(
                ArithmeticOperator::Multiplication,
                Box::new(ArithmeticExpression::Integer(6)),
                Box::new(ArithmeticExpression::Unary(
                    UnaryArithmeticOperator::Negation,
                    Box::new(ArithmeticExpression::Integer(7)),
                )),
            )),
            Box::new(ArithmeticExpression::Integer(9)),
        )
    );

    let tokens = tokeniser.tokenise("5 == 5".to_string()).unwrap();
    let result = parse_relations(&mut tokens.into());
    assert_eq!(
        result.unwrap(),
        BooleanExpression::Relational(
            RelationOperator::Equal,
            Box::new(ArithmeticExpression::Integer(5)),
            Box::new(ArithmeticExpression::Integer(5)),
        )
    );

    let tokens = tokeniser.tokenise("5 == 5 && 5 != 8".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        BooleanExpression::Logical(
            BooleanOperator::And,
            Box::new(BooleanExpression::Relational(
                RelationOperator::Equal,
                Box::new(ArithmeticExpression::Integer(5)),
                Box::new(ArithmeticExpression::Integer(5)),
            )),
            Box::new(BooleanExpression::Relational(
                RelationOperator::NotEqual,
                Box::new(ArithmeticExpression::Integer(5)),
                Box::new(ArithmeticExpression::Integer(8)),
            )),
        )
    );

    let tokens = tokeniser.tokenise("5 == 5 || 5 != 8".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        BooleanExpression::Logical(
            BooleanOperator::Or,
            Box::new(BooleanExpression::Relational(
                RelationOperator::Equal,
                Box::new(ArithmeticExpression::Integer(5)),
                Box::new(ArithmeticExpression::Integer(5)),
            )),
            Box::new(BooleanExpression::Relational(
                RelationOperator::NotEqual,
                Box::new(ArithmeticExpression::Integer(5)),
                Box::new(ArithmeticExpression::Integer(8)),
            )),
        )
    );

    let tokens = tokeniser.tokenise("!4 == 5".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        BooleanExpression::Unary(
            UnaryBooleanOperator::Negation,
            Box::new(BooleanExpression::Relational(
                RelationOperator::Equal,
                Box::new(ArithmeticExpression::Integer(4)),
                Box::new(ArithmeticExpression::Integer(5)),
            )),
        )
    );

    let tokens = tokeniser
        .tokenise("!(4 == 5) && (5 == 5 || 4 != 5)".to_string())
        .unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    assert_eq!(
        result.unwrap(),
        BooleanExpression::Logical(
            BooleanOperator::And,
            Box::new(BooleanExpression::Unary(
                UnaryBooleanOperator::Negation,
                Box::new(BooleanExpression::Relational(
                    RelationOperator::Equal,
                    Box::new(ArithmeticExpression::Integer(4)),
                    Box::new(ArithmeticExpression::Integer(5)),
                )),
            )),
            Box::new(BooleanExpression::Logical(
                BooleanOperator::Or,
                Box::new(BooleanExpression::Relational(
                    RelationOperator::Equal,
                    Box::new(ArithmeticExpression::Integer(5)),
                    Box::new(ArithmeticExpression::Integer(5)),
                )),
                Box::new(BooleanExpression::Relational(
                    RelationOperator::NotEqual,
                    Box::new(ArithmeticExpression::Integer(4)),
                    Box::new(ArithmeticExpression::Integer(5)),
                )),
            )),
        )
    );
}

#[test]
fn parse_programs() {
    let while_program = "int x = 5;
    while (x > 0) do {
        int y = 5;
        while (y > 0) do {
            if 4 == 5 then {
                y = y - 1;
            } else {
                y = y - 1;
            };
        };
        x = x - 1;
    };";

    let mut tokeniser = crate::lexer::Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(while_program.to_string()).unwrap();

    let result = crate::parser::parse_program(&mut tokens.into());

    assert_eq!(
        result.unwrap(),
        Program::Statements(vec![
            Statement::Assignment(Assignment::Integer(
                "x".to_string(),
                Box::new(ArithmeticExpression::Integer(5))
            )),
            Statement::While(
                Box::new(BooleanExpression::Relational(
                    RelationOperator::GreaterThan,
                    Box::new(ArithmeticExpression::Variable("x".to_string())),
                    Box::new(ArithmeticExpression::Integer(0))
                )),
                Box::new(vec![
                    Statement::Assignment(Assignment::Integer(
                        "y".to_string(),
                        Box::new(ArithmeticExpression::Integer(5))
                    )),
                    Statement::While(
                        Box::new(BooleanExpression::Relational(
                            RelationOperator::GreaterThan,
                            Box::new(ArithmeticExpression::Variable("y".to_string())),
                            Box::new(ArithmeticExpression::Integer(0))
                        )),
                        Box::new(vec![Statement::If(
                            Box::new(BooleanExpression::Relational(
                                RelationOperator::Equal,
                                Box::new(ArithmeticExpression::Integer(4)),
                                Box::new(ArithmeticExpression::Integer(5))
                            )),
                            Box::new(vec![Statement::Assignment(Assignment::Integer(
                                "y".to_string(),
                                Box::new(ArithmeticExpression::Operation(
                                    ArithmeticOperator::Subtraction,
                                    Box::new(ArithmeticExpression::Variable("y".to_string())),
                                    Box::new(ArithmeticExpression::Integer(1))
                                ))
                            ))]),
                            Box::new(vec![Statement::Assignment(Assignment::Integer(
                                "y".to_string(),
                                Box::new(ArithmeticExpression::Operation(
                                    ArithmeticOperator::Subtraction,
                                    Box::new(ArithmeticExpression::Variable("y".to_string())),
                                    Box::new(ArithmeticExpression::Integer(1))
                                ))
                            ))])
                        )])
                    ),
                    Statement::Assignment(Assignment::Integer(
                        "x".to_string(),
                        Box::new(ArithmeticExpression::Operation(
                            ArithmeticOperator::Subtraction,
                            Box::new(ArithmeticExpression::Variable("x".to_string())),
                            Box::new(ArithmeticExpression::Integer(1))
                        ))
                    ))
                ])
            )
        ])
    );
}
