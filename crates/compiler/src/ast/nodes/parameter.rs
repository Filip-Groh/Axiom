use crate::ast::{IdentifierNode, Node};
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct ParameterNode {
    location: Range,
    pub identifier_node: Box<IdentifierNode>,
    pub type_node: Box<IdentifierNode>,
}

impl ParameterNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, type_node: Box<IdentifierNode>) -> ParameterNode {
        ParameterNode {
            location,
            identifier_node,
            type_node,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- {}: {}", " ".repeat(indent * 4), self.identifier_node.identifier_token.name, self.type_node.identifier_token.name);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.identifier_node.location()) {
            return self.identifier_node.get_node_at(position);
        }

        self.type_node.get_node_at(position)
    }
}

impl Location for ParameterNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}