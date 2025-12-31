use crate::ast::{IdentifierNode, Node, ParameterNode, ScopeNode};
use crate::datatype::DataType;
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct FunctionNode {
    location: Range,
    pub data_type: DataType,
    pub identifier_node: Box<IdentifierNode>,
    pub parameters: Vec<Box<ParameterNode>>,
    pub type_node: Option<Box<IdentifierNode>>,
    pub scope: Box<ScopeNode>,
}

impl FunctionNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, parameters: Vec<Box<ParameterNode>>, type_node: Option<Box<IdentifierNode>>, scope: Box<ScopeNode>) -> FunctionNode {
        FunctionNode {
            location,
            data_type: DataType::Function(vec![], Box::from(DataType::None)),
            identifier_node,
            parameters,
            type_node,
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

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }
        
        if position.is_in_range(&self.identifier_node.location()) {
            return self.identifier_node.get_node_at(position);
        }

        if let Some(node) = self.parameters.iter().map(|parameter_node| parameter_node.get_node_at(position)).filter(|node| node.is_some()).next() {
            return node;
        }

        if let Some(type_node) = &self.type_node && position.is_in_range(&type_node.location()) {
            return type_node.get_node_at(position);
        }

        self.scope.get_node_at(position)
    }
}

impl Location for FunctionNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}