use std::fmt;

#[derive(Debug)]
pub struct LoxError {
    line: usize,
    place: String,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, place: Option<&str>, message: &str) -> Self {
        Self {
            line,
            place: place.unwrap_or("").to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error{}: {}", self.line, self.place, self.message)
    }
}

impl std::error::Error for LoxError {}
