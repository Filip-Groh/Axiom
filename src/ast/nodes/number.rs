use crate::token::{NumberToken};

pub struct NumberNode {
    pub number_token: NumberToken
}

impl NumberNode {
    pub fn new(number_token: NumberToken) -> NumberNode {
        NumberNode {
            number_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.number_token.value);
    }
}