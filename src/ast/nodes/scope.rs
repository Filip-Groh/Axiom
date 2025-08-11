use crate::ast::Node;

pub struct ScopeNode {
    pub statements: Vec<Box<Node>>
}

impl ScopeNode {
    pub fn new(statements: Vec<Box<Node>>) -> ScopeNode {
        ScopeNode {
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
}