use crate::ast::{IdentifierNode, Node};

pub struct FunctionNode {
    pub identifier_node: Box<IdentifierNode>,
    pub parameters: Vec<Box<IdentifierNode>>,
    pub scope: Box<Node>,
}

impl FunctionNode {
    pub fn new(identifier_node: Box<IdentifierNode>, parameters: Vec<Box<IdentifierNode>>, scope: Box<Node>) -> FunctionNode {
        FunctionNode {
            identifier_node,
            parameters,
            scope
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- function {}(", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        for param in &self.parameters {
            param.display(indent + 1);
        }
        println!("{})", " ".repeat(indent * 4));
        self.scope.display(indent + 1);
    }
}