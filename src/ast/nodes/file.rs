use crate::ast::Node;

pub struct FileNode {
    pub functions: Vec<Box<Node>>
}

impl FileNode {
    pub fn new(functions: Vec<Box<Node>>) -> FileNode {
        FileNode {
            functions
        }
    }
    
    pub fn display(&self, indent: usize) {
        for function in &self.functions {
            function.display(indent);
        }
    }
}