pub use crate::ast::nodes::*;
use crate::error::location::Location;

mod nodes;

pub enum Node {
    File(Location, FileNode),
    Number(Location, NumberNode),
    Identifier(Location, IdentifierNode),
    BinaryOperation(Location, Box<BinaryOperationNode>),
    Assignment(Location, Box<AssignmentNode>),
    Scope(Location, ScopeNode),
    Function(Location, Box<FunctionNode>),
    Return(Location, Box<ReturnNode>),
}

impl Node {
    pub fn display(&self, indent: usize) {
        match self {
            Node::File(_, file_node) => file_node.display(indent),
            Node::Number(_, number_node) => number_node.display(indent),
            Node::Identifier(_, identifier_node) => identifier_node.display(indent),
            Node::BinaryOperation(_, binary_operation_node) => binary_operation_node.display(indent),
            Node::Assignment(_, assignment_node) => assignment_node.display(indent),
            Node::Scope(_, scope_node) => scope_node.display(indent),
            Node::Function(_, function_node) => function_node.display(indent),
            Node::Return(_, return_node) => return_node.display(indent),
        }
    }
    
    pub fn location(&self) -> &Location {
            match self {
                Node::File(location, _) => location,
                Node::Number(location, _) => location,
                Node::Identifier(location, _) => location,
                Node::BinaryOperation(location, _) => location,
                Node::Assignment(location, _) => location,
                Node::Scope(location, _) => location,
                Node::Function(location, _) => location,
                Node::Return(location, _) => location,
            }
        }
}