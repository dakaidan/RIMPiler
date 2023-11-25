mod regex;
#[cfg(test)]
mod tests;

use crate::lexer::regex::LexError;
use super::re::{*};

pub trait Token {
    fn new(string: String, record_identifier: String) -> Self;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 0,
        }
    }
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}

pub struct TokenMeta<T>
    where T: Token
{
    pub token: T,
    pub lexeme: String,
    pub location: Location,
}

impl<T> TokenMeta<T>
    where T: Token
{
    pub fn new(lexeme: String, location: Location, record_identifier: String) -> Self {
        Self {
            token: T::new(lexeme.clone(), record_identifier),
            lexeme,
            location,
        }
    }
}

struct Lexer {
    regex: Re,
}

impl Lexer {
    fn new(regex: Re) -> Self {
        Self {
            regex,
        }
    }

    fn tokenise<T>(&self, input: &str) -> Result<Vec<TokenMeta<T>>, LexError>
        where T: Token
    {
       let result =  self.regex.lex(input.to_owned());

        match result {
            Ok(environment) => {
                environment.iter().map(|(record_identifier, lexeme, location)| {
                    Ok(TokenMeta::new(lexeme.clone(), *location, record_identifier.clone()))
                }).collect()
            }
            Err(error) => {
                panic!("Lexing failed: {:?}", error);
            }
        }
    }
}