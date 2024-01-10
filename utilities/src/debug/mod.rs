use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 0 }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.line, self.column)
    }
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}