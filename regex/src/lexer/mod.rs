mod regex;
#[cfg(test)]
mod tests;

use super::re::*;
use super::lexer::regex::LexError;
use utilities::debug::Location;


pub trait Token: Clone + Eq {
    fn new(string: String, record_identifier: String) -> Result<Box<Self>, String>;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenMeta<T>
where
    T: Token,
{
    pub token: T,
    pub lexeme: String,
    pub location: Location,
}

impl<T> TokenMeta<T>
where
    T: Token,
{
    pub fn new(
        lexeme: String,
        location: Location,
        record_identifier: String,
    ) -> Result<Self, String> {
        let token = T::new(lexeme.clone(), record_identifier);
        match token {
            Ok(token) => Ok(Self {
                token: *token,
                lexeme,
                location,
            }),
            Err(error) => Err(error),
        }
    }
}

pub struct Lexer {
    regex: Re,
}

impl Lexer {
    pub fn new(regex: Re) -> Self {
        Self { regex }
    }

    pub fn tokenise<T>(&self, input: &str) -> Result<Vec<TokenMeta<T>>, LexError>
    where
        T: Token,
    {
        let result = self.regex.lex(input.to_owned());

        match result {
            Ok(environment) => environment
                .iter()
                .map(|(record_identifier, lexeme, location)| {
                    let result =
                        TokenMeta::new(lexeme.clone(), *location, record_identifier.clone());
                    match result {
                        Ok(token_meta) => Ok(token_meta),
                        Err(error) => Err(LexError::new(error, *location)),
                    }
                })
                .collect(),
            Err(error) => Err(error),
        }
    }
}
