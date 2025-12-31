use crate::ast::Node;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct ScopeNode {
    location: Range,
    pub statements: Vec<Box<Node>>
}

impl ScopeNode {
    pub fn new(location: Range, statements: Vec<Box<Node>>) -> ScopeNode {
        ScopeNode {
            location,
            statements
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {{", " ".repeat(indent * 4));
        for statement in &self.statements {
            statement.display(indent + 1);
        }
        println!("{}}}", " ".repeat(indent * 4));
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        self.statements.iter().map(|node| node.get_node_at(position)).filter(|node| node.is_some()).next()?
    }
}

impl Location for ScopeNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}