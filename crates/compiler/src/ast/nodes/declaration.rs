use crate::ast::{IdentifierNode, Node};
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct DeclarationNode {
    location: Range,
    pub identifier_node: Box<IdentifierNode>,
    pub expression: Box<Node>,
}

impl DeclarationNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, expression: Box<Node>) -> DeclarationNode {
        DeclarationNode {
            location,
            identifier_node,
            expression,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- let {} = ", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        self.expression.display(indent * 4);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.identifier_node.location()) {
            return self.identifier_node.get_node_at(position);
        }

        self.expression.get_node_at(position)
    }
}

impl Location for DeclarationNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}