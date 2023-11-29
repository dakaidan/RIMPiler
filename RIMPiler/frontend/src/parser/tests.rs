use crate::parser::{parse_arithmetic_expression, parse_relations, parse_boolean_expression, parse_statement, parse_block, parse_program};

#[test]
fn basic_parse() {
    // TODO: Convert to real tests, and add significant examples that are not so obvious
    let mut tokeniser = crate::lexer::Tokeniser::new().initialise();

    let tokens = tokeniser.tokenise("9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("1 + 2".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("1 + 2 * 3".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("1 + 2 ^ 5 + 8 * 3".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("-1".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("6 * -7^9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("(6 * -7) + 9".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("(7 * 8) / 14 + 4".to_string()).unwrap();
    let result = parse_arithmetic_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("5 == 5".to_string()).unwrap();
    let result = parse_relations(&mut tokens.into());
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("5 == 5 && 5 != 8".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("5 == 5 || 5 != 8".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("5 == 5 || 5 != 8 && 5 == 5".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("5 * (6 / 2)^9 == 1".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("!4 == 5".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("!(4 == 5)".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("!(4 == 5) && (5 == 5 || 4 != 5)".to_string()).unwrap();
    let result = parse_boolean_expression(&mut tokens.into(), 0);
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("skip".to_string()).unwrap();
    let result = parse_statement(&mut tokens.into());
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("int i = 6".to_string()).unwrap();
    let result = parse_statement(&mut tokens.into());
    println!("{}", result.unwrap());

    let tokens = tokeniser.tokenise("{int i = 6;}".to_string()).unwrap();
    let result = parse_block(&mut tokens.into());
    println!("{:?}", result.unwrap());

    let tokens = tokeniser.tokenise("{ int i = 6; int j = 7; }".to_string()).unwrap();
    let result = parse_block(&mut tokens.into());
    println!("{:?}", result.unwrap());

    let tokens = tokeniser.tokenise("if a == 6 then { int a = 9; } else { int a = 0; };".to_string()).unwrap();
    let result = parse_statement(&mut tokens.into());
    println!("{}\n", result.unwrap());

    let tokens = tokeniser.tokenise("while a == 6 do { int a = 9; };".to_string()).unwrap();
    let result = parse_statement(&mut tokens.into());
    println!("{}\n", result.unwrap());

    let tokens = tokeniser.tokenise("while a > 0 do { if a == 4 then { b = 0; } else { b = b; }; }; };".to_string()).unwrap();
    let result = parse_statement(&mut tokens.into());
    println!("{}\n", result.unwrap());

    let tokens = tokeniser.tokenise("int a = 0; while a == 0 do { x = y; }; y = a;".to_string()).unwrap();
    let result = parse_program(&mut tokens.into());
    println!("{}\n", result.unwrap());
}