pub use crate::ast::nodes::*;

mod nodes;

pub enum Node {
    Number(NumberNode),
    Identifier(IdentifierNode),
    BinaryOperation(Box<BinaryOperationNode>),
    Assignment(Box<AssignmentNode>),
    Scope(ScopeNode),
    Function(Box<FunctionNode>)
}

impl Node {
    pub fn display(&self, indent: usize) {
        match self {
            Node::Number(number_node) => number_node.display(indent),
            Node::Identifier(identifier_node) => identifier_node.display(indent),
            Node::BinaryOperation(binary_operation_node) => binary_operation_node.display(indent),
            Node::Assignment(assignment_node) => assignment_node.display(indent),
            Node::Scope(scope_node) => scope_node.display(indent),
            Node::Function(function_node) => function_node.display(indent),
        }
    }
}