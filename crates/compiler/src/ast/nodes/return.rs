use crate::ast::Node;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct ReturnNode {
    location: Range,
    pub expression: Box<Node>,
}

impl ReturnNode {
    pub fn new(location: Range, expression: Box<Node>) -> ReturnNode {
        ReturnNode { 
            location,
            expression
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- return ", " ".repeat(indent * 4));
        self.expression.display(indent + 1);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        self.expression.get_node_at(position)
    }
}

impl Location for ReturnNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}