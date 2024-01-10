use std::fmt::Display;
use regex::lexer::{Token, TokenMeta};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
            _ => unreachable!(
                "Should only be called by the Lexer, invalid keyword, {}",
                string
            ),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
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
    Not,
}

impl Operator {
    fn new(string: String) -> Self {
        match string.as_str() {
            "+" => Operator::Add,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "^" => Operator::Exponent,
            "==" => Operator::Equal,
            "=" => Operator::Assign,
            "<" => Operator::LessThan,
            ">" => Operator::GreaterThan,
            "!=" => Operator::NotEqual,
            "&&" => Operator::And,
            "||" => Operator::Or,
            "!" => Operator::Not,
            _ => unreachable!(
                "Should only be called by the Lexer, invalid binary operator, {}",
                string
            ),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Operator::Add => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Exponent => "^",
            Operator::Equal => "==",
            Operator::Assign => "=",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::NotEqual => "!=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Not => "!",
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
            _ => unreachable!(
                "Should only be called by the Lexer, invalid bracket, {}",
                string
            ),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RIMPToken {
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
    Number(i32), // I believe PISA only supports 32-bit integers, if so, no need to lex larger
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

    // a clone function which uses copy as much as possible, unless
    // it is an identifier, in which case it clones the string
    fn copy_clone(&self) -> Self {
        match self {
            RIMPToken::Keyword(keyword) => RIMPToken::Keyword(*keyword),
            RIMPToken::Identifier(identifier) => RIMPToken::Identifier(identifier.clone()),
            RIMPToken::Operator(binary_operator) => RIMPToken::Operator(*binary_operator),
            RIMPToken::Number(number) => RIMPToken::Number(*number),
            RIMPToken::Bracket(bracket) => RIMPToken::Bracket(*bracket),
            RIMPToken::Semicolon => RIMPToken::Semicolon,
            RIMPToken::Whitespace => RIMPToken::Whitespace,
            RIMPToken::Comment => RIMPToken::Comment,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<TokenMeta<RIMPToken>>,
}

impl Tokens {
    pub fn new(tokens: Vec<TokenMeta<RIMPToken>>) -> Tokens {
        let mut toks = tokens.clone();
        toks.reverse();
        Tokens { tokens: toks }
    }

    pub fn next(&mut self) -> Option<TokenMeta<RIMPToken>> {
        self.tokens.pop()
    }

    // Since we need strings, this is the best case cloning we can do
    pub fn peek(&self) -> Option<TokenMeta<RIMPToken>> {
        match self.tokens.last() {
            Some(token) => Some(TokenMeta {
                token: token.token.copy_clone(),
                location: token.location,
                lexeme: token.lexeme.clone(),
            }),
            None => None,
        }
    }
}

impl From<Vec<TokenMeta<RIMPToken>>> for Tokens {
    fn from(tokens: Vec<TokenMeta<RIMPToken>>) -> Self {
        Tokens::new(tokens)
    }
}

impl Token for RIMPToken {
    fn new(string: String, record_identifier: String) -> Result<Box<Self>, String> {
        let tok = match record_identifier.as_str() {
            "keyword" => Ok(RIMPToken::Keyword(Keyword::new(string))),
            "identifier" => Ok(RIMPToken::Identifier(string)),
            "operator" => Ok(RIMPToken::Operator(Operator::new(string))),
            "number" => {
                let number = RIMPToken::parse_number(string);
                match number {
                    Ok(number) => Ok(RIMPToken::Number(number)),
                    Err(error) => Err(error),
                }
            }
            "bracket" => Ok(RIMPToken::Bracket(Bracket::new(string))),
            "semicolon" => Ok(RIMPToken::Semicolon),
            "whitespace" => Ok(RIMPToken::Whitespace),
            "comment" => Ok(RIMPToken::Comment),
            _ => unreachable!(
                "Should only be called by the Lexer, invalid token, {}",
                string
            ),
        };

        match tok {
            Ok(tok) => Ok(Box::new(tok)),
            Err(error) => Err(error),
        }
    }
}
