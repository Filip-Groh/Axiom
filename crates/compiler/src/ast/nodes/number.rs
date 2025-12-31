use crate::ast::Node;
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};
use crate::token::NumberToken;

#[derive(Clone)]
#[derive(Debug)]
pub struct NumberNode {
    location: Range,
    pub data_type: DataType,
    pub number_token: NumberToken
}

impl NumberNode {
    pub fn new(location: Range, number_token: NumberToken) -> NumberNode {
        NumberNode {
            location,
            data_type: DataType::I32,
            number_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.number_token.value);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        Some(Box::from(Node::Number(self.clone())))
    }
}

impl Location for NumberNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}