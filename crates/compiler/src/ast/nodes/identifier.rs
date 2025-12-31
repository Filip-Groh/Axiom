use crate::ast::Node;
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};
use crate::token::IdentifierToken;

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub data_type: DataType,
    pub identifier_token: IdentifierToken,
}

impl IdentifierNode {
    pub fn new(identifier_token: IdentifierToken) -> IdentifierNode {
        IdentifierNode {
            data_type: DataType::ToBeInferred,
            identifier_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.identifier_token.name);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        Some(Box::from(Node::Identifier(self.clone())))
    }
}

impl Location for IdentifierNode {
    fn location(&self) -> Range {
        self.identifier_token.location()
    }
}