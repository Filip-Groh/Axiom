use crate::ast::{IdentifierNode, ParameterNode};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{PunctuationType, Token};

impl Parser {
    pub fn parameter(&mut self) -> Result<Box<ParameterNode>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        if let Token::Identifier(identifier_token) = token {
            let identifier_node = IdentifierNode::new(identifier_token);

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Colon)) {
                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ':'".into()))
            }

            let colon_location = token.location();

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            if let Token::Identifier(type_identifier_token) = token {
                let type_identifier_node = IdentifierNode::new(type_identifier_token);

                self.step();

                let location = Range::from_ranges(vec![identifier_node.location(), colon_location, type_identifier_node.location()]);
                let parameter_node = ParameterNode::new(location, Box::from(identifier_node), Box::from(type_identifier_node));

                Ok(Box::from(parameter_node))
            } else {
                Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
            }
        } else {
            Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
        }
    }
}