use crate::error::location::{Location, Position, Range};
use crate::token::IdentifierToken;

#[derive(Debug, PartialEq, Clone)]
pub enum PunctuationType {
    Comma,
    Colon
}

#[derive(Debug, PartialEq, Clone)]
pub struct PunctuationToken {
    pub punctuation_type: PunctuationType,
    location: Position
}

impl PunctuationToken {
    pub fn new(punctuation_type: PunctuationType, location: Position) -> PunctuationToken {
        PunctuationToken {
            punctuation_type,
            location
        }
    }
}

impl Location for PunctuationToken {
    fn location(&self) -> Range {
        Range {
            start: self.location.clone(),
            end: self.location.clone()
        }
    }
}