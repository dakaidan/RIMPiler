use super::super::lexer::Tokeniser;
use super::super::parser::parse;
use super::super::post_parse::inverter::invert;
use super::super::AST::{
    ArithmeticExpression, ArithmeticOperator, Assignment, BooleanExpression, Program,
    RelationOperator, Statement,
};

#[test]
fn name_generator() {
    use super::name_generator::NameGenerator;

    let mut name_generator = NameGenerator::new(String::from("test"));

    assert_eq!(name_generator.generate(), "generated_name_test0");
    assert_eq!(name_generator.generate(), "generated_name_test1");
    assert_eq!(name_generator.generate(), "generated_name_test2");
}

#[test]
fn transformer() {
    let program = r#"
        while (1 < 2) do {
            skip;
        };
    "#;

    let mut tokeniser = Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(program.to_string()).unwrap();

    let ast = parse(&mut tokens.into()).unwrap();

    assert_eq!(
        ast,
        Program::Statements(vec![
            Statement::Assignment(Assignment::Integer(
                String::from("generated_name_semantic_transformer0"),
                Box::new(ArithmeticExpression::Integer(0))
            )),
            Statement::While(
                Box::new(BooleanExpression::Relational(
                    RelationOperator::LessThan,
                    Box::new(ArithmeticExpression::Integer(1)),
                    Box::new(ArithmeticExpression::Integer(2))
                )),
                Box::new(vec![
                    Statement::Skip,
                    Statement::Assignment(Assignment::Integer(
                        String::from("generated_name_semantic_transformer0"),
                        Box::new(ArithmeticExpression::Operation(
                            ArithmeticOperator::Addition,
                            Box::new(ArithmeticExpression::Variable(String::from(
                                "generated_name_semantic_transformer0"
                            ))),
                            Box::new(ArithmeticExpression::Integer(1))
                        ))
                    ))
                ])
            )
        ])
    )
}

#[test]
fn inverter() {
    let program = r#"
        while (1 < 2) do {
            if (1 < 2) then {
                skip;
            } else {
                skip;
            };
        };
    "#;

    let mut tokeniser = Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(program.to_string()).unwrap();

    let ast = parse(&mut tokens.into()).unwrap();

    let reversed_ast = invert(&ast);

    assert_eq!(
        reversed_ast,
        Program::Statements(vec![
            Statement::While(
                Box::new(BooleanExpression::Relational(
                    RelationOperator::GreaterThan,
                    Box::new(ArithmeticExpression::Variable(String::from(
                        "generated_name_semantic_transformer0"
                    ))),
                    Box::new(ArithmeticExpression::Integer(0))
                )),
                Box::new(vec![
                    Statement::ReverseAssignment(Assignment::Integer(
                        String::from("generated_name_semantic_transformer0"),
                        Box::new(ArithmeticExpression::Operation(
                            ArithmeticOperator::Addition,
                            Box::new(ArithmeticExpression::Variable(String::from(
                                "generated_name_semantic_transformer0"
                            ))),
                            Box::new(ArithmeticExpression::Integer(1))
                        ))
                    )),
                    Statement::Skip,
                    Statement::If(
                        Box::new(BooleanExpression::Relational(
                            RelationOperator::LessThan,
                            Box::new(ArithmeticExpression::Integer(1)),
                            Box::new(ArithmeticExpression::Integer(2))
                        )),
                        Box::new(vec![Statement::Skip]),
                        Box::new(vec![Statement::Skip])
                    )
                ])
            ),
            Statement::Skip,
            Statement::ReverseAssignment(Assignment::Integer(
                String::from("generated_name_semantic_transformer0"),
                Box::new(ArithmeticExpression::Integer(0))
            ))
        ])
    )
}
