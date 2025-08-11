use crate::ast::Node;

pub struct ReturnNode {
    pub expression: Box<Node>,
}

impl ReturnNode {
    pub fn new(expression: Box<Node>) -> ReturnNode {
        ReturnNode { 
            expression
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- return ", " ".repeat(indent * 4));
        self.expression.display(indent + 1);
    }
}