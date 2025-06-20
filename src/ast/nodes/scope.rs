use crate::ast::Node;

pub struct ScopeNode {
    statements: Vec<Node>,
}

impl ScopeNode {
    pub fn new(statements: Vec<Node>) -> ScopeNode {
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