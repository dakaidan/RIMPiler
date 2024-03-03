use crate::interpreter::memory_store::Value;
use super::super::interpreter::InterpreterEngine;
use super::super::lexer::Tokeniser;
use super::super::parser::parse;
use super::super::post_parse::inverter::invert_and_combine;

#[test]
fn basic_interpreted() {
    let program = r#"
        while (1 < 1) do {
            skip;
        };
    "#;

    let mut tokeniser = Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(program.to_string()).unwrap();

    let ast = parse(&mut tokens.into()).unwrap();
    let inverted = invert_and_combine(&ast.clone());

    let mut interpreter = InterpreterEngine::new();
    let result = interpreter.interpret(&inverted);

    assert_eq!(result, Ok(()));
    assert_eq!(
        interpreter
            .get_result(&String::from("generated_name_semantic_transformer0"))
            .unwrap()
            .get(),
        Value::Integer(0)
    );
}

#[test]
fn result() {
    let program = r#"
        int n = 5;
        int minus1 = 1;
        int minus2 = 0;
        while n > 0 do {
               int temp = minus2;
               minus2 = minus1 + minus2;
               minus1 = temp;
               n = n - 1;
        };
    "#;

    let mut tokeniser = Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(program.to_string()).unwrap();

    let ast = parse(&mut tokens.into()).unwrap();
    let inverted = invert_and_combine(&ast.clone());

    let mut interpreter = InterpreterEngine::new();
    let result = interpreter.interpret(&inverted);

    assert_eq!(result, Ok(()));
    assert_eq!(
        interpreter
            .get_result(&String::from("minus2"))
            .unwrap()
            .get(),
        Value::Integer(5)
    );

    let program = r#"
        int n = 10;
        int minus1 = 1;
        int minus2 = 0;
        while n > 0 do {
               int temp = minus2;
               minus2 = minus1 + minus2;
               minus1 = temp;
               n = n - 1;
        };
    "#;

    let mut tokeniser = Tokeniser::new().initialise();
    let tokens = tokeniser.tokenise(program.to_string()).unwrap();

    let ast = parse(&mut tokens.into()).unwrap();
    let inverted = invert_and_combine(&ast.clone());

    let mut interpreter = InterpreterEngine::new();
    let result = interpreter.interpret(&inverted);

    assert_eq!(result, Ok(()));
    assert_eq!(
        interpreter
            .get_result(&String::from("minus2"))
            .unwrap()
            .get(),
        Value::Integer(55)
    );
}
