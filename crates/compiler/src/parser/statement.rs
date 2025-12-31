use crate::ast::{CallNode, DeclarationNode, IdentifierNode, IfElseNode, Node, ReturnNode, ScopeNode, UnaryNode, UnaryType};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{KeywordType, OperatorArithmeticType, OperatorAssignmentType, OperatorCategory, ParenthesesState, ParenthesesType, PunctuationType, Token};

impl Parser {
    pub fn statement(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Let) => {
                self.step();

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                match token {
                    Token::Identifier(identifier_token) => {
                        let identifier_node = IdentifierNode::new(identifier_token);

                        self.step();

                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                        match token {
                            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                                self.step();

                                let expression = self.expression()?;
                                let expression_location = expression.location();

                                let location = Range::from_ranges(vec![keyword_token.location(), identifier_node.location(), operator_token.location(), expression_location.clone()]);
                                let declaration_node = DeclarationNode::new(location, Box::from(identifier_node), expression);
                                let node = Node::Declaration(declaration_node);

                                Ok(Box::from(node))
                            }
                            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '='".into()))
                        }
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                }
            }
            Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Return) => {
                self.step();

                let expression = self.expression()?;

                let location = Range::from_ranges(vec![keyword_token.location(), expression.location()]);
                let return_node = ReturnNode::new(location, expression);
                let node = Node::Return(return_node);

                Ok(Box::from(node))
            }
            Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::If) => {
                self.step();

                let condition = self.expression()?;
                let consequent = self.scope()?;

                let mut alternative = None;
                let mut conditional_alternatives: Vec<(Box<Node>, Box<ScopeNode>)> = Vec::new();
                let mut locations = vec![keyword_token.location(), condition.location(), consequent.location()];

                loop {
                    let token = self.current_token.clone();

                    match token {
                        Some(token) if matches!(&token, Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Else)) => {
                            locations.push(token.location());

                            self.step();

                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                            if !matches!(&token, Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::If)) {
                                let alternative_node = self.scope()?;

                                locations.push(alternative_node.location());
                                alternative = Some(alternative_node);

                                break;
                            }

                            locations.push(token.location());

                            self.step();

                            let condition = self.expression()?;
                            let consequent = self.scope()?;

                            locations.push(condition.location());
                            locations.push(consequent.location());

                            conditional_alternatives.push((condition, consequent));
                        }
                        _ => {
                            break;
                        }
                    }
                }

                let location = Range::from_ranges(locations);
                let if_else_node = IfElseNode::new(location, condition, consequent, conditional_alternatives, alternative);
                let node = Node::IfElse(if_else_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_token) => {
                let identifier_node = IdentifierNode::new(identifier_token);

                self.step();

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                match token {
                    Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Increment)) => {
                        self.step();

                        let location = Range::from_ranges(vec![identifier_node.location(), operator_token.location()]);
                        let unary_node = UnaryNode::new(location, Box::from(Node::Identifier(identifier_node)), UnaryType::PostIncrement);
                        let node = Node::Unary(unary_node);

                        Ok(Box::new(node))
                    },
                    Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Arithmetic(OperatorArithmeticType::Decrement)) => {
                        self.step();

                        let location = Range::from_ranges(vec![identifier_node.location(), operator_token.location()]);
                        let unary_node = UnaryNode::new(location, Box::from(Node::Identifier(identifier_node)), UnaryType::PostDecrement);
                        let node = Node::Unary(unary_node);

                        Ok(Box::new(node))
                    },
                    Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                        self.step();

                        let mut parameters = vec![];
                        let mut parameter_locations = vec![identifier_node.location(), parentheses_token.location()];

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

                        Ok(Box::from(node))
                    }
                    _ => {
                        let assignment = self.assignment(Box::from(identifier_node))?;

                        Ok(assignment)
                    }
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected tokens".into()))
        }
    }
}