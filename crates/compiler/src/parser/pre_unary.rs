use crate::ast::{Node, UnaryNode, UnaryType};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::error::location::{Location, Range};
use crate::token::{OperatorArithmeticType, OperatorCategory, OperatorLogicalType, Token};

impl Parser {
    pub fn pre_unary(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Increment)) => {
                self.step();

                let expression = self.pre_unary()?;

                let location = Range::from_ranges(vec![expression.location(), operator_token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::PreIncrement);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Decrement)) => {
                self.step();

                let expression = self.pre_unary()?;

                let location = Range::from_ranges(vec![expression.location(), operator_token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::PreDecrement);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Addition)) => {
                self.step();

                let expression = self.pre_unary()?;

                let location = Range::from_ranges(vec![expression.location(), operator_token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::Absolute);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction)) => {
                self.step();

                let expression = self.pre_unary()?;

                let location = Range::from_ranges(vec![expression.location(), operator_token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::Minus);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Logical(OperatorLogicalType::Not)) => {
                self.step();

                let expression = self.pre_unary()?;

                let location = Range::from_ranges(vec![expression.location(), operator_token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::Not);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            _ => {
                self.post_unary()
            }
        }
    }
}