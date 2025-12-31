use crate::ast::{FunctionNode, Node};
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct FileNode {
    location: Range,
    pub functions: Vec<Box<FunctionNode>>
}

impl FileNode {
    pub fn new(location: Range, functions: Vec<Box<FunctionNode>>) -> FileNode {
        FileNode {
            location,
            functions
        }
    }
    
    pub fn display(&self, indent: usize) {
        for function in &self.functions {
            function.display(indent);
        }
    }
    
    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }
        
        self.functions.iter().map(|function_node| function_node.get_node_at(position)).filter(|node| node.is_some()).next()?
    }
}

impl Location for FileNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}