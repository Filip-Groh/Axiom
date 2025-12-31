use crate::ast::{IdentifierNode, Node};
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct CallNode {
    pub(crate) location: Range,
    pub data_type: DataType,
    pub identifier_node: Box<IdentifierNode>,
    pub parameters: Vec<Box<Node>>,
}

impl CallNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, parameters: Vec<Box<Node>>) -> CallNode {
        CallNode {
            location,
            data_type: DataType::ToBeInferred,
            identifier_node,
            parameters,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- {}(", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        for param in &self.parameters {
            param.display(indent + 1);
        }
        println!("{})", " ".repeat(indent * 4));
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.identifier_node.location()) {
            return self.identifier_node.get_node_at(position);
        }

        self.parameters.iter().map(|parameter_node| parameter_node.get_node_at(position)).filter(|node| node.is_some()).next()?
    }
}

impl Location for CallNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}