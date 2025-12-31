use crate::ast::{CallNode, IdentifierNode, Node, NumberNode};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{ParenthesesState, ParenthesesType, PunctuationType, Token};

impl Parser {
    pub fn primary(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Number(number_token) => {
                self.step();

                let number_node = NumberNode::new(number_token.location(), number_token);
                let node = Node::Number(number_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_token) => {
                self.step();

                let identifier_node = IdentifierNode::new(identifier_token);

                if let Some(token) = self.current_token.clone() && matches!(&token, Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening))) {
                    self.step();

                    let mut parameters = vec![];
                    let mut parameter_locations = vec![identifier_node.location(), token.location()];

                    let mut first_loop = true;
                    loop {
                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                        if let Token::Parentheses(parentheses_token) = &token && matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                            parameter_locations.push(parentheses_token.location());

                            self.step();

                            break
                        }

                        if !first_loop {
                            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                            }

                            parameter_locations.push(token.location());

                            self.step()
                        }

                        let expression = self.expression()?;
                        let expression_location = expression.location();

                        parameters.push(expression);
                        parameter_locations.push(expression_location);

                        first_loop = false;
                    }

                    let location = Range::from_ranges(parameter_locations);
                    let call_node = CallNode::new(location, Box::from(identifier_node), parameters);
                    let node = Node::Call(call_node);

                    return Ok(Box::from(node))
                }

                let node = Node::Identifier(identifier_node);

                Ok(Box::from(node))
            }
            Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                self.step();

                let expression = self.expression()?;

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                match token {
                    Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) => {
                        self.step();

                        Ok(expression)
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ')'".into()))
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected tokens".into()))
        }
    }
}