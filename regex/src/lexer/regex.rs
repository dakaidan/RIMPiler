use super::Location;
use super::super::re::{Re, value::Value};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LexError {
    pub message: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct LexResult {
    pub value: Value,
}

impl LexResult {
    pub fn new(value: Value) -> Self {
        Self {
            value,
        }
    }

    pub fn environment(&self) -> Vec<(String, String)> {
        self.value.environment()
    }
}

pub(crate) fn to_line_column(string: &str, line: usize, column: usize) -> (usize, usize) {
    let mut line = line;
    let mut column = column;
    for c in string.chars() {
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    (line, column)
}

impl Re {
    fn try_lex(&self, string: String, column: usize, line: usize) -> Result<LexResult, LexError> {
        println!("{}: {} -> {}", line, column, string);
        if string.is_empty() {
            if self.nullable() {
                Ok(
                    LexResult::new(
                        self.make_empty(),
                    )
                )
            } else {
                Err(LexError {
                    message: format!("Expected EOF at line {}, column {}", line, column),
                    location: Location::new(line, column)
                })
            }
        } else {
            let (c, mut remaining) = string.split_at(1);
            let c = c.chars().next().unwrap();
            let derivative = self.derivative(c);
            let (simplified, rectification) = derivative.simplify_with_rectification();
            if simplified == Re::Zero {
                Err(LexError {
                    message: format!("Unexpected character '{}' at line {}, column {}", c, line, column),
                    location: Location::new(line, column)
                })
            } else {
                let mut line = line;
                let mut column = column;
                if string.starts_with("\r\n") {
                    remaining = &remaining[1..];
                    line += 1;
                    column = 0;
                } else if c == '\n' {
                    line += 1;
                    column = 0;
                } else {
                    column += 1;
                }

                let result = simplified.try_lex(remaining.to_owned(), column, line);

                println!("{}: {} -> {}", line, column, c);

                match result {
                    Ok(value) => Ok(
                        LexResult::new(
                            self.injection(c, &mut rectification(value.value)),
                        )
                    ),
                    Err(error) => Err(error),
                }
            }
        }
    }

    pub(crate) fn lex(&self, string: String) -> Result<Vec<(String, String, Location)>, LexError> {
        match self.try_lex(string, 0, 1) {
            Ok(value) => {
                let mut line = 1;
                let mut column = 0;
                Ok(
                    value.environment().iter().map(|(record_identifier, lexeme)| {
                        (line, column) = to_line_column(lexeme, line, column);
                        (record_identifier.clone(), lexeme.clone(), Location::new(line, column))
                    }).collect()
                )
            },
            Err(error) => Err(error),
        }
    }
}