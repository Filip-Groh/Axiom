use crate::ast::Node;

#[derive(Debug)]
pub enum BinaryOperationType {
    Addition(),
    Subtraction(),
    Multiplication(),
    Division()
}

pub struct BinaryOperationNode {
    pub left: Node,
    pub right: Node,
    operation_type: BinaryOperationType,
}

impl BinaryOperationNode {
    pub fn new(left: Node, right: Node, operation_type: BinaryOperationType) -> BinaryOperationNode {
        BinaryOperationNode {
            left,
            right,
            operation_type
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {:?}", " ".repeat(indent * 4), self.operation_type);
        self.left.display(indent + 1);
        self.right.display(indent + 1);
    }
}