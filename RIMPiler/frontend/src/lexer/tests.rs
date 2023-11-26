use regex::lexer::{Location, TokenMeta};
use regex::re::Range::Range;
use regex::re::Re;
use utilities::files::load_file;
use crate::lexer::tokens::{Keyword, RIMPToken};
use super::Tokeniser;

#[test]
fn simple_lex() {
    let result = Tokeniser::new().initialise().tokenise_without_filtering("skip".to_owned());

    assert_eq!(
        result,
        Ok(vec![TokenMeta {
            token: RIMPToken::Keyword(Keyword::Skip),
            lexeme: "skip".to_string(),
            location: Location { line: 1, column: 0 }
        }
        ])
    );

    let result = Tokeniser::new().initialise().tokenise_without_filtering("if n\nthen\n\tskip\nelse\n\tskip".to_owned());

    assert_eq!(
        result,
        Ok(vec![
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::If),
                lexeme: "if".to_string(),
                location: Location { line: 1, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: " ".to_string(),
                location: Location { line: 1, column: 2 }
            },
            TokenMeta {
                token: RIMPToken::Identifier("n".to_string()),
                lexeme: "n".to_string(),
                location: Location { line: 1, column: 3 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\n".to_string(),
                location: Location { line: 1, column: 4 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Then),
                lexeme: "then".to_string(),
                location: Location { line: 2, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\n".to_string(),
                location: Location { line: 2, column: 4 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\t".to_string(),
                location: Location { line: 3, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Skip),
                lexeme: "skip".to_string(),
                location: Location { line: 3, column: 1 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\n".to_string(),
                location: Location { line: 3, column: 5 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Else),
                lexeme: "else".to_string(),
                location: Location { line: 4, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\n".to_string(),
                location: Location { line: 4, column: 4 }
            },
            TokenMeta {
                token: RIMPToken::Whitespace,
                lexeme: "\t".to_string(),
                location: Location { line: 5, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Skip),
                lexeme: "skip".to_string(),
                location: Location { line: 5, column: 1 }
            },
        ])
    );
}

#[test]
fn filtered_lex() {
    let result = Tokeniser::new().initialise().tokenise(
        "if n\nthen\n\tskip\nelse\n\tskip".to_owned(),
    );

    assert_eq!(
        result,
        Ok(vec![
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::If),
                lexeme: "if".to_string(),
                location: Location { line: 1, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Identifier("n".to_string()),
                lexeme: "n".to_string(),
                location: Location { line: 1, column: 3 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Then),
                lexeme: "then".to_string(),
                location: Location { line: 2, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Skip),
                lexeme: "skip".to_string(),
                location: Location { line: 3, column: 1 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Else),
                lexeme: "else".to_string(),
                location: Location { line: 4, column: 0 }
            },
            TokenMeta {
                token: RIMPToken::Keyword(Keyword::Skip),
                lexeme: "skip".to_string(),
                location: Location { line: 5, column: 1 }
            },
        ])
    );
}

#[test]
fn collatz_lex() {
    let collatz = load_file("../../examples/collatz.rimp").unwrap();

    let result = Tokeniser::new().initialise().tokenise(collatz);

    println!("collatz:\n{:?}", result);
}