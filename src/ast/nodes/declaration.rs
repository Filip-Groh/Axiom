use crate::ast::{IdentifierNode, Node};

pub struct DeclarationNode {
    pub identifier_node: Box<IdentifierNode>,
    pub expression: Box<Node>,
}

impl DeclarationNode {
    pub fn new(identifier_node: Box<IdentifierNode>, expression: Box<Node>) -> DeclarationNode {
        DeclarationNode {
            identifier_node,
            expression,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- let {} = ", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        self.expression.display(indent * 4);
    }
}