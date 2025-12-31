use crate::ast::Node;
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct TernaryNode {
    location: Range,
    pub data_type: DataType,
    pub condition: Box<Node>,
    pub consequent: Box<Node>,
    pub alternative: Box<Node>,
}

impl TernaryNode {
    pub fn new(location: Range, condition: Box<Node>, consequent: Box<Node>, alternative: Box<Node>) -> TernaryNode {
        TernaryNode {
            location,
            data_type: DataType::ToBeInferred,
            condition,
            consequent,
            alternative,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- ", " ".repeat(indent * 4));
        self.condition.display(indent + 1);
        println!("{}- ? ", " ".repeat(indent * 4));
        self.consequent.display(indent + 1);
        println!("{}- : ", " ".repeat(indent * 4));
        self.alternative.display(indent + 1);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.condition.location()) {
            return self.condition.get_node_at(position);
        }

        if position.is_in_range(&self.consequent.location()) {
            return self.consequent.get_node_at(position);
        }

        self.alternative.get_node_at(position)
    }
}

impl Location for TernaryNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}