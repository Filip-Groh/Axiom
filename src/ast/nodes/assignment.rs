use crate::ast::{IdentifierNode, Node};

pub struct AssignmentNode {
    pub identifier_node: IdentifierNode,
    pub expression: Node,
}

impl AssignmentNode {
    pub fn new(identifier_node: IdentifierNode, expression: Node) -> AssignmentNode {
        AssignmentNode {
            identifier_node,
            expression
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- let {} = ", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        self.expression.display(indent + 1);
    }
}