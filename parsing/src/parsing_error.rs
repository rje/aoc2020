use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub fn new(s: &str) -> ParseError {
        ParseError {
            details: s.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
