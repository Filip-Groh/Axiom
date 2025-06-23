use crate::error::location::Location;
use crate::token::{IdentifierToken};

pub struct IdentifierNode {
    pub location: Location,
    pub identifier_token: IdentifierToken,
}

impl IdentifierNode {
    pub fn new(location: Location, identifier_token: IdentifierToken) -> IdentifierNode {
        IdentifierNode {
            location,
            identifier_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.identifier_token.name);
    }
}