use regex::lexer::Token;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Keyword {
    Skip,
    If,
    Then,
    Else,
    While,
    Do,
    Int,
}

impl Keyword {
    fn new(string: String) -> Self {
        match string.as_str() {
            "skip" => Keyword::Skip,
            "if" => Keyword::If,
            "then" => Keyword::Then,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "do" => Keyword::Do,
            "int" => Keyword::Int,
            _ => unreachable!("Should only be called by the Lexer, invalid keyword, {}", string),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Equal,
    Assign,
    LessThan,
    GreaterThan,
    NotEqual,
    And,
    Or,
}

impl BinaryOperator {
    fn new(string: String) -> Self {
        match string.as_str() {
            "+" => BinaryOperator::Plus,
            "-" => BinaryOperator::Minus,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            "^" => BinaryOperator::Exponent,
            "==" => BinaryOperator::Equal,
            "=" => BinaryOperator::Assign,
            "<" => BinaryOperator::LessThan,
            ">" => BinaryOperator::GreaterThan,
            "!=" => BinaryOperator::NotEqual,
            "&&" => BinaryOperator::And,
            "||" => BinaryOperator::Or,
            _ => unreachable!("Should only be called by the Lexer, invalid binary operator, {}", string),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
}

impl UnaryOperator {
    fn new(string: String) -> Self {
        match string.as_str() {
            "!" => UnaryOperator::Not,
            _ => unreachable!("Should only be called by the Lexer, invalid unary operator, {}", string),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Bracket {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}

impl Bracket {
    fn new(string: String) -> Self {
        match string.as_str() {
            "(" => Bracket::LeftParenthesis,
            ")" => Bracket::RightParenthesis,
            "{" => Bracket::LeftBrace,
            "}" => Bracket::RightBrace,
            _ => unreachable!("Should only be called by the Lexer, invalid bracket, {}", string),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RIMPToken {
    Keyword(Keyword),
    Identifier(String),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    Number(i32),    // I believe PISA only supports 32-bit integers, if so, no need to lex larger
    Bracket(Bracket),
    Semicolon,
    Whitespace,
    Comment,
}

impl RIMPToken {
    fn parse_number(string: String) -> Result<i32, String> {
        let parsed = string.parse::<i32>();
        match parsed {
            Ok(number) => Ok(number),
            Err(_) => return Err(format!("Invalid 32 bit number, {}", string)),
        }
    }
}

impl Token for RIMPToken {
    fn new(string: String, record_identifier: String) -> Result<Box<Self>, String> {
        let tok = match record_identifier.as_str() {
            "keyword" => Ok(RIMPToken::Keyword(Keyword::new(string))),
            "identifier" => Ok(RIMPToken::Identifier(string)),
            "binary operator" => Ok(RIMPToken::BinaryOperator(BinaryOperator::new(string))),
            "unary operator" => Ok(RIMPToken::UnaryOperator(UnaryOperator::new(string))),
            "number" => {
                let number = RIMPToken::parse_number(string);
                match number {
                    Ok(number) => Ok(RIMPToken::Number(number)),
                    Err(error) => Err(error),
                }
            },
            "bracket" => Ok(RIMPToken::Bracket(Bracket::new(string))),
            "semicolon" => Ok(RIMPToken::Semicolon),
            "whitespace" => Ok(RIMPToken::Whitespace),
            "comment" => Ok(RIMPToken::Comment),
            _ => unreachable!("Should only be called by the Lexer, invalid token, {}", string),
        };

        match tok {
            Ok(tok) => Ok(Box::new(tok)),
            Err(error) => Err(error),
        }
    }
}