use crate::ast::{AssignmentNode, BinaryType, IdentifierNode, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::error::location::{Location, Range};
use crate::token::{OperatorAssignmentType, OperatorCategory, Token};

impl Parser {
    pub fn assignment(&mut self, identifier_node: Box<IdentifierNode>) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                self.step();

                let expression = self.expression()?;
                let expression_location = expression.location();

                let location = Range::from_ranges(vec![identifier_node.location(), operator_token.location(), expression_location.clone()]);
                let assignment_node = AssignmentNode::new(location, Box::from(identifier_node), expression);
                let node = Node::Assignment(assignment_node);

                Ok(Box::from(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::AdditionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::Addition, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::SubtractionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::Subtraction, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::MultiplicationAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::Multiplication, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::DivisionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::Division, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::ShiftLeftAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::ShiftLeft, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::ShiftRightAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::ShiftRight, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::BitwiseOrAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::BitwiseOr, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::BitwiseAndAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::BitwiseAnd, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::OrAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::Or, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::AndAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryType::And, |_self| {_self.expression()})?)
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '='".into()))
        }
    }
}