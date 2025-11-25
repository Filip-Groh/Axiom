use crate::ast::{Node, UnaryNode, UnaryType};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{OperatorArithmeticType, OperatorCategory, Token};

impl Parser {
    pub fn post_unary(&mut self) -> Result<Box<Node>, AxiomError> {
        let expression = self.primary()?;

        let token = self.current_token.clone();

        match token {
            Some(token) if matches!(&token, Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Increment))) => {
                self.step();

                let location = Range::from_ranges(vec![expression.location(), token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::PostIncrement);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            Some(token) if matches!(&token, Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Decrement))) => {
                self.step();

                let location = Range::from_ranges(vec![expression.location(), token.location()]);
                let unary_node = UnaryNode::new(location, expression, UnaryType::PostDecrement);
                let node = Node::Unary(unary_node);

                Ok(Box::new(node))
            }
            _ => Ok(expression)
        }
    }
}