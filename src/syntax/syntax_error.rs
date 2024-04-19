use std::fmt;

use text_size::{TextRange, TextSize};

/// Represents an error that can happen during parsing or lexing
/// This can be also used in further AST validations to throw another error
/// Each error has a message and a range
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, TextRange);

impl SyntaxError {
    /// Create a new error with a message and a range
    pub fn new(message: impl Into<String>, range: TextRange) -> Self {
        Self(message.into(), range)
    }

    /// Create a new error with a message and an offset
    pub fn new_at_offset(message: impl Into<String>, offset: TextSize) -> Self {
        Self(message.into(), TextRange::empty(offset))
    }

    /// Get the range
    pub fn range(&self) -> TextRange {
        self.1
    }

    /// Connect range to the error
    pub fn with_range(mut self, range: TextRange) -> Self {
        self.1 = range;
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
