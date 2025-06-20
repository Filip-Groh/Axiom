use crate::token::{IdentifierToken, Location};

pub struct IdentifierNode {
    location: Location,
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