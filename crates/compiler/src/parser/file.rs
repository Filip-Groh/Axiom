use crate::ast::{FileNode, FunctionNode, IdentifierNode, Node};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{KeywordType, ParenthesesState, ParenthesesType, PunctuationType, Token};

impl Parser {
    pub fn file(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut functions = vec![];
        let mut file_locations = vec![];

        loop {
            let token = match self.current_token.clone() {
                Some(token) => token,
                None => break,
            };

            match token {
                Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Function) => {
                    self.step();

                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                    match token {
                        Token::Identifier(identifier_token) => {
                            let identifier_node = IdentifierNode::new(identifier_token);

                            self.step();

                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                            match token {
                                Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                                    self.step();

                                    let mut parameters = vec![];
                                    let mut locations = vec![keyword_token.location(), identifier_node.location(), parentheses_token.location()];

                                    let mut first_loop = true;
                                    loop {
                                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                        if let Token::Parentheses(parentheses_token) = &token {
                                            if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                                                locations.push(parentheses_token.location());

                                                self.step();

                                                break
                                            }
                                        }

                                        if !first_loop {
                                            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                                            }

                                            locations.push(token.location());

                                            self.step()
                                        }

                                        let parameter = self.parameter()?;
                                        let parameter_location = parameter.location();

                                        parameters.push(parameter);
                                        locations.push(parameter_location);

                                        first_loop = false;
                                    }

                                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                    let mut output_type = None;
                                    if let Token::Punctuation(punctuation_token) = &token {
                                        if matches!(punctuation_token.punctuation_type, PunctuationType::Colon) {
                                            self.step();

                                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                            if let Token::Identifier(type_identifier_token) = token {
                                                let type_identifier_location = type_identifier_token.location();
                                                let type_identifier_node = IdentifierNode::new(type_identifier_token);

                                                self.step();

                                                locations.push(punctuation_token.location());
                                                locations.push(type_identifier_location);

                                                output_type = Some(Box::from(type_identifier_node));
                                            } else {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                                            }
                                        }
                                    }

                                    let scope = self.scope()?;
                                    let scope_location = scope.location();
                                    locations.push(scope_location.clone());

                                    let location = Range::from_ranges(locations);
                                    let function_node = FunctionNode::new(location.clone(), Box::from(identifier_node), parameters, output_type, scope);

                                    file_locations.push(location.clone());
                                    functions.push(Box::from(function_node));
                                }
                                _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '('".into()))
                            }
                        }
                        _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                    }
                }
                _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected tokens".into()))
            }
        }

        let location = Range::from_ranges(file_locations);
        let file_node = FileNode::new(location, functions);
        let node = Node::File(file_node);

        Ok(Box::from(node))
    }
}