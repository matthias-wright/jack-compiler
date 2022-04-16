//! Defines parse errors.
use std::fmt;
use std::fmt::Formatter;

/// This error occurs when the Jack code violates the syntax rules.
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}}}", file!(), line!())
    }
}