use crate::ast::{Node, ScopeNode};
use crate::error::location::{Location, Position, Range};

#[derive(Debug)]
pub struct IfElseNode {
    location: Range,
    pub condition: Box<Node>,
    pub consequent: Box<ScopeNode>,
    pub conditional_alternatives: Vec<(Box<Node>, Box<ScopeNode>)>,
    pub alternative: Option<Box<ScopeNode>>,
}

impl IfElseNode {
    pub fn new(location: Range, condition: Box<Node>, consequent: Box<ScopeNode>, conditional_alternatives: Vec<(Box<Node>, Box<ScopeNode>)>, alternative: Option<Box<ScopeNode>>) -> IfElseNode {
        IfElseNode {
            location,
            condition,
            consequent,
            conditional_alternatives,
            alternative,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- if ", " ".repeat(indent * 4));
        self.condition.display(indent + 1);
        self.consequent.display(indent + 1);

        for conditional_alternative in &self.conditional_alternatives {
            println!("{}- else if ", " ".repeat(indent * 4));
            conditional_alternative.0.display(indent + 1);
            conditional_alternative.1.display(indent + 1);
        }

        if let Some(alternative) = &self.alternative {
            println!("{}- else ", " ".repeat(indent * 4));
            alternative.display(indent + 1);
        }
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.condition.location()) {
            return self.condition.get_node_at(position);
        }

        if position.is_in_range(&self.consequent.location()) {
            return self.consequent.get_node_at(position);
        }

        if let Some(node) = self.conditional_alternatives.iter().map(|(condition_node, consequent_node)| vec![condition_node.get_node_at(position), consequent_node.get_node_at(position)]).flatten().filter(|node| node.is_some()).next() {
            return node;
        }
        
        if let Some(node) = &self.alternative {
            node.get_node_at(position)
        } else {
            None
        }
    }
}

impl Location for IfElseNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}