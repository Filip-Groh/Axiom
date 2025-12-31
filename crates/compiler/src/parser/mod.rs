mod file;
mod parameter;
mod scope;
mod statement;
mod assignment;
mod expression;
mod ternary;
mod conditional_or;
mod conditional_and;
mod equality;
mod bitwise;
mod shift;
mod additive;
mod multiplicative;
mod pre_unary;
mod post_unary;
mod primary;

use crate::ast::{AssignmentNode, BinaryNode, BinaryType, IdentifierNode, Node};
use crate::error::{AxiomError};
use crate::error::location::{Location, Position, Range};
use crate::token::{Token};

pub struct Parser {
    index: usize,
    current_token: Option<Token>,
    previous_token: Option<Token>,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            current_token: tokens.get(0).cloned(),
            previous_token: None,
            tokens
        }
    }
    
    fn step(&mut self) {
        self.previous_token = self.current_token.clone();
        self.index += 1;
        self.current_token = self.tokens.get(self.index).cloned();
    }

    fn get_next_position_from_last_token_location(&self) -> Position {
        let last_token_end_position = self.previous_token.clone().unwrap_or(Token::Unknown(Position::new(0, 0), ' ')).location().end;
        Position {
            line: last_token_end_position.line,
            column: last_token_end_position.column + 1
        }
    }

    fn get_current_location_from_current_token(&self) -> Range {
        self.current_token.clone().unwrap_or(Token::Unknown(Position::new(0, 0), ' ')).location()
    }
    
    pub fn parse(mut self) -> Result<Box<Node>, AxiomError> {
        self.file()
    }

    fn binary_operation<F>(&mut self, left: Box<Node>, operator_location: Range, binary_operation_type: BinaryType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();
        
        let right = right_fn(self)?;
        let right_location = right.location();

        let location = Range::from_ranges(vec![left.location(), operator_location, right_location]);
        let binary_operation_node = BinaryNode::new(location, left, right, binary_operation_type);
        let node = Node::Binary(binary_operation_node);

        Ok(Box::from(node))
    }

    fn assignment_operation<F>(&mut self, identifier_node: Box<IdentifierNode>, operator_location: Range, binary_operation_type: BinaryType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();

        let right = right_fn(self)?;
        let right_location = right.location();

        let location = Range::from_ranges(vec![identifier_node.location(), operator_location.clone(), right_location.clone()]);
        let expression_node = BinaryNode::new(location.clone(), Box::from(Node::Identifier(*identifier_node.clone())), right, binary_operation_type);

        let assignment_node = AssignmentNode::new(location, identifier_node, Box::from(Node::Binary(expression_node)));
        let node = Node::Assignment(assignment_node);

        Ok(Box::from(node))
    }
}