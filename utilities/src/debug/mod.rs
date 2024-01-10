use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    unknown: bool
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unknown {
            write!(f, "[unknown]")
        } else {
            write!(f, "[{}, {}]", self.line, self.column)
        }
    }
}


impl Default for Location {
    fn default() -> Self {
        Self { line: 0, column: 0, unknown: true }
    }
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column, unknown: false }
    }

    pub fn is_unknown(&self) -> bool {
        self.unknown
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Error {
    pub location: Location,
    pub message: String,
    pub system: String
}

impl Default for Error {
    fn default() -> Self {
        Self {
            location: Location::default(),
            message: String::from("Unknown error"),
            system: String::from("Unknown system")
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = format!("{}", self.location);
        let message = format!("{}", self.message);
        let system = format!("{}", self.system);

        write!(f, "{}: {} Error:\n{}", location, system, message)
    }
}

impl Error {
    pub fn new(location: Location, message: String, system: String) -> Self {
        Self { location, message, system }
    }

    pub fn from_error(error: Error, message: String, system: String) -> Self {
        Self {
            location: error.location,
            message: format!("{}\n from:\n{}", message, error.message),
            system: format!("{}::{}", system, error.system)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Meta<T> {
    pub value: T,
    pub location: Location
}

impl<T> Meta<T> {
    pub fn new(value: T, location: Location) -> Self {
        Self { value, location }
    }

    pub fn to_error(self, message: String, system: String) -> Error {
        Error::new(self.location, message, system)
    }
}

impl<T> Display for Meta<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;