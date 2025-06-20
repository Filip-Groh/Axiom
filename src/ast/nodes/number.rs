use crate::token::{Location, NumberToken};

pub struct NumberNode {
    location: Location,
    number_token: NumberToken
}

impl NumberNode {
    pub fn new(location: Location, number_token: NumberToken) -> NumberNode {
        NumberNode {
            location,
            number_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.number_token.value);
    }
}