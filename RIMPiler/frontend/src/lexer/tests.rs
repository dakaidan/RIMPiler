use super::lexer::Tokeniser;
use super::super::lexer::tokens::{Bracket, Keyword, Operator, RIMPToken};

use utilities::debug::{Location, Meta};

#[test]
fn simple_lex() {
    let result = Tokeniser::new()
        .initialise()
        .tokenise_without_filtering("skip".to_owned());

    assert_eq!(
        result,
        Ok(vec![Meta {
            value: RIMPToken::Keyword(Keyword::Skip),
            location: Location::new(1, 0)
        }])
    );

    let result = Tokeniser::new()
        .initialise()
        .tokenise_without_filtering("if n\nthen\n\tskip\nelse\n\tskip".to_owned());

    assert_eq!(
        result,
        Ok(vec![
            Meta {
                value: RIMPToken::Keyword(Keyword::If),
                location: Location::new(1, 0)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(1, 2)
            },
            Meta {
                value: RIMPToken::Identifier("n".to_string()),
                location: Location::new(1, 3)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(1, 4)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Then),
                location: Location::new(2, 0)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(2, 4)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(3, 0)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Skip),
                location: Location::new(3, 1)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(3, 5)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Else),
                location: Location::new(4, 0)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(4, 4)
            },
            Meta {
                value: RIMPToken::Whitespace,
                location: Location::new(5, 0)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Skip),
                location: Location::new(5, 1)
            },
        ])
    );
}

#[test]
fn filtered_lex() {
    let result = Tokeniser::new()
        .initialise()
        .tokenise("if n\nthen\n\tskip\nelse\n\tskip".to_owned());

    assert_eq!(
        result,
        Ok(vec![
            Meta {
                value: RIMPToken::Keyword(Keyword::If),
                location: Location::new(1, 0)
            },
            Meta {
                value: RIMPToken::Identifier("n".to_string()),
                location: Location::new(1, 3)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Then),
                location: Location::new(2, 0)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Skip),
                location: Location::new(3, 1)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Else),
                location: Location::new(4, 0)
            },
            Meta {
                value: RIMPToken::Keyword(Keyword::Skip),
                location: Location::new(5, 1)
            },
        ])
    );
}

#[test]
fn collatz_lex() {
    let collatz = r#"
    /*
        Example RIMP program to check if a number is a Collatz number.
        In RIMP we will require variables to be declared before use.
        They will be declared in the form:
        <type> <name> = <value>;
        where <type> is currently only int, <name> is the name of the variable,
        and <value> is the value to be assigned to the variable.
        The value can be an integer literal, or another variable.

        If collatz = 0, then n is not a Collatz number.
        If collatz = 1, then n is a Collatz number.
    */

    int n = 1977931; // number to be checked
    int collatz = 0;

    while n > 1 do {
      // --- We have no modulo operator in RIMP, so this calculates the remainder ---
      int q = n / 2;
      int p = q * 2;
      int r = n - p;
      // --- End of remainder calculation ---
      if r == 0
      then {n = n/2;}
      else {n = 3*n+1;};
    };

    collatz = 1;
    "#;

    let result = Tokeniser::new().initialise().tokenise(collatz.to_string());

    let result = result
        .unwrap()
        .iter()
        .map(|x| x.value.clone())
        .collect::<Vec<_>>();

    assert_eq!(
        result,
        vec![
            RIMPToken::Keyword(Keyword::Int),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Integer(1977931),
            RIMPToken::Semicolon,
            RIMPToken::Keyword(Keyword::Int),
            RIMPToken::Identifier("collatz".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Integer(0),
            RIMPToken::Semicolon,
            RIMPToken::Keyword(Keyword::While),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::GreaterThan),
            RIMPToken::Integer(1),
            RIMPToken::Keyword(Keyword::Do),
            RIMPToken::Bracket(Bracket::LeftBrace),
            RIMPToken::Keyword(Keyword::Int),
            RIMPToken::Identifier("q".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Divide),
            RIMPToken::Integer(2),
            RIMPToken::Semicolon,
            RIMPToken::Keyword(Keyword::Int),
            RIMPToken::Identifier("p".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Identifier("q".to_string()),
            RIMPToken::Operator(Operator::Multiply),
            RIMPToken::Integer(2),
            RIMPToken::Semicolon,
            RIMPToken::Keyword(Keyword::Int),
            RIMPToken::Identifier("r".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Minus),
            RIMPToken::Identifier("p".to_string()),
            RIMPToken::Semicolon,
            RIMPToken::Keyword(Keyword::If),
            RIMPToken::Identifier("r".to_string()),
            RIMPToken::Operator(Operator::Equal),
            RIMPToken::Integer(0),
            RIMPToken::Keyword(Keyword::Then),
            RIMPToken::Bracket(Bracket::LeftBrace),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Divide),
            RIMPToken::Integer(2),
            RIMPToken::Semicolon,
            RIMPToken::Bracket(Bracket::RightBrace),
            RIMPToken::Keyword(Keyword::Else),
            RIMPToken::Bracket(Bracket::LeftBrace),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Integer(3),
            RIMPToken::Operator(Operator::Multiply),
            RIMPToken::Identifier("n".to_string()),
            RIMPToken::Operator(Operator::Add),
            RIMPToken::Integer(1),
            RIMPToken::Semicolon,
            RIMPToken::Bracket(Bracket::RightBrace),
            RIMPToken::Semicolon,
            RIMPToken::Bracket(Bracket::RightBrace),
            RIMPToken::Semicolon,
            RIMPToken::Identifier("collatz".to_string()),
            RIMPToken::Operator(Operator::Assign),
            RIMPToken::Integer(1),
            RIMPToken::Semicolon,
        ]
    )
}
