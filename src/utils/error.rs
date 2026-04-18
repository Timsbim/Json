use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    InvalidString(Option<usize>),
    InvalidToken((Option<usize>, &'static str)),
    InvalidValue(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JsonError::*;

        match self {
            InvalidString(Some(position)) => write!(f, "invalid character at position {position}"),
            InvalidString(_) => write!(f, "invalid JSON string"),
            InvalidToken((Some(p), token)) => write!(f, "invalid token at position {p}: {token}"),
            InvalidToken((_, message)) => write!(f, "invalid token: {message}"),
            InvalidValue(value) => write!(f, "invalid value: `{value}`"),
        }
    }
}

impl Error for JsonError {}
