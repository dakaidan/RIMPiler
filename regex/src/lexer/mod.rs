mod regex;
#[cfg(test)]
mod tests;

use super::re::*;
use utilities::debug::{Error, Location, Meta, Result};


pub trait Token: Clone + Eq {
    fn new(string: String, record_identifier: String) -> std::result::Result<Box<Self>, String>;
}

pub trait TokenMetaTrait<T>
where
    T: Token,
    Self: Sized,
{
    fn create_token(
        lexeme: String,
        location: Location,
        record_identifier: String,
    ) -> std::result::Result<Self, String>;
}

impl<T> TokenMetaTrait<T> for Meta<T>
where
    T: Token,
{
    fn create_token(
        lexeme: String,
        location: Location,
        record_identifier: String,
    ) -> std::result::Result<Self, String> {
        let token = T::new(lexeme.clone(), record_identifier);
        match token {
            Ok(token) => Ok(Self {
                value: *token,
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

    pub fn tokenise<T>(&self, input: &str) -> Result<Vec<Meta<T>>>
    where
        T: Token,
    {
        let result = self.regex.lex(input.to_owned());

        match result {
            Ok(environment) => environment
                .iter()
                .map(|(record_identifier, lexeme, location)| {
                    let result =
                        Meta::create_token(lexeme.clone(), *location, record_identifier.clone());
                    match result {
                        Ok(token_meta) => Ok(token_meta),
                        Err(error) => Err(Error::new(*location, error, "Lexer".to_string())),
                    }
                })
                .collect(),
            Err(error) => Err(error),
        }
    }
}
