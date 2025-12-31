use crate::error::location::{Location, Position, Range};

#[derive(Debug, PartialEq, Clone)]
pub enum ParenthesesState {
    Opening,
    Closing
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParenthesesType {
    Round(ParenthesesState),
    Curly(ParenthesesState)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParenthesesToken {
    pub parentheses_type: ParenthesesType,
    location: Position
}

impl ParenthesesToken {
    pub fn new(parentheses_type: ParenthesesType, location: Position) -> ParenthesesToken {
        ParenthesesToken {
            parentheses_type,
            location
        }
    }
}

impl Location for ParenthesesToken {
    fn location(&self) -> Range {
        Range {
            start: self.location.clone(),
            end: self.location.clone()
        }
    }
}