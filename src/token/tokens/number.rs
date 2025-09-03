use crate::error::location::{Location, Range};
use crate::token::IdentifierToken;

#[derive(Debug, PartialEq, Clone)]
pub struct NumberToken {
    pub value: String,
    location: Range
}

impl NumberToken {
    pub fn new(value: String, location: Range) -> NumberToken {
        NumberToken {
            value,
            location
        }
    }
}

impl Location for NumberToken {
    fn location(&self) -> Range {
        self.location.clone()
    }
}