use crate::ast::Node;
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub enum UnaryType {
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
    Minus,
    Absolute,
    Not
}

#[derive(Debug)]
pub struct UnaryNode {
    location: Range,
    pub data_type: DataType,
    pub expression: Box<Node>,
    pub operation_type: UnaryType,
}

impl UnaryNode {
    pub fn new(location: Range, expression: Box<Node>, operation_type: UnaryType) -> UnaryNode {
        UnaryNode {
            location,
            data_type: DataType::ToBeInferred,
            expression,
            operation_type
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- {:?}", " ".repeat(indent * 4), self.operation_type);
        self.expression.display(indent + 1);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        self.expression.get_node_at(position)
    }
}

impl Location for UnaryNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}