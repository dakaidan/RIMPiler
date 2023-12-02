use super::regex::*;
use super::*;

#[test]
fn to_line_column_test() {
    let string = "abc\ndef\nghi";
    assert_eq!(to_line_column(string, 0, 0), (2, 3));
    assert_eq!(to_line_column(string, 1, 0), (3, 3));
    assert_eq!(to_line_column(string, 2, 5), (4, 3));

    let string = "abc";
    assert_eq!(to_line_column(string, 0, 0), (0, 3));
    assert_eq!(to_line_column(string, 0, 5), (0, 8));
    assert_eq!(to_line_column(string, 5, 5), (5, 8));

    let string = "";
    assert_eq!(to_line_column(string, 0, 0), (0, 0));

    let string = "\n";
    assert_eq!(to_line_column(string, 0, 0), (1, 0));
    assert_eq!(to_line_column(string, 1, 0), (2, 0));
    assert_eq!(to_line_column(string, 0, 5), (1, 0));
}

#[test]
fn lex() {
    let re = Re::star(Re::alt(
        Re::record("0".to_owned(), Re::char('a')),
        Re::record("1".to_owned(), Re::char('b')),
    ));
    let result = re.lex("ab".to_owned());
    assert_eq!(
        result,
        Ok(vec![
            ("0".to_owned(), "a".to_owned(), Location::new(1, 0)),
            ("1".to_owned(), "b".to_owned(), Location::new(1, 1))
        ])
    );
}

#[test]
fn tokenise() {
    #[derive(Clone, Eq, PartialEq, Debug)]
    enum TestToken {
        A,
        B,
    }

    impl Token for TestToken {
        fn new(_: String, record_identifier: String) -> Result<Box<Self>, String> {
            match record_identifier.as_str() {
                "0" => Ok(Box::new(TestToken::A)),
                "1" => Ok(Box::new(TestToken::B)),
                _ => Err(format!("Unknown record identifier: {}", record_identifier)),
            }
        }
    }

    let re = Re::star(Re::alt(
        Re::record("0".to_owned(), Re::char('a')),
        Re::record("1".to_owned(), Re::char('b')),
    ));

    let lexer = Lexer::new(re);
    let result = lexer.tokenise::<TestToken>("ab");
    assert_eq!(
        result,
        Ok(vec![
            TokenMeta::new("a".to_owned(), Location::new(1, 0), "0".to_owned()).unwrap(),
            TokenMeta::new("b".to_owned(), Location::new(1, 1), "1".to_owned()).unwrap(),
        ])
    );
}
