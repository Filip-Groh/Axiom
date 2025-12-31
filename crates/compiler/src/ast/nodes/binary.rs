use crate::ast::Node;
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub enum BinaryType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    ShiftLeft,
    ShiftRight,
    BitwiseOr,
    BitwiseAnd,
    Or,
    And,
}

#[derive(Debug)]
pub struct BinaryNode {
    location: Range,
    pub data_type: DataType,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub operation_type: BinaryType,
}

impl BinaryNode {
    pub fn new(location: Range, left: Box<Node>, right: Box<Node>, operation_type: BinaryType) -> BinaryNode {
        BinaryNode {
            location,
            data_type: DataType::ToBeInferred,
            left,
            right,
            operation_type
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {:?}", " ".repeat(indent * 4), self.operation_type);
        self.left.display(indent + 1);
        self.right.display(indent + 1);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.left.location()) {
            return self.left.get_node_at(position);
        }

        self.right.get_node_at(position)
    }
}

impl Location for BinaryNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}