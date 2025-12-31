use crate::error::location::{Location, Range};

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierToken {
    pub name: String,
    location: Range
}

impl IdentifierToken {
    pub fn new(name: String, location: Range) -> IdentifierToken {
        IdentifierToken {
            name,
            location
        }
    }
}

impl Location for IdentifierToken {
    fn location(&self) -> Range {
        self.location.clone()
    }
}