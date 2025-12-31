use crate::ast::{Node, TernaryNode};
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::parser::Parser;
use crate::token::{PunctuationType, Token};

impl Parser {
    pub fn ternary(&mut self) -> Result<Box<Node>, AxiomError> {
        let condition = self.conditional_or()?;

        let token = self.current_token.clone();

        match token {
            None => Ok(condition),
            Some(token) => {
                if !matches!(&token, Token::Punctuation(punctuation_token) if punctuation_token.punctuation_type == PunctuationType::QuestionMark) {
                    return Ok(condition);
                }

                let question_mark_token = token;

                self.step();

                let consequent = self.expression()?;

                let colon_token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;
                if !matches!(&colon_token, Token::Punctuation(punctuation_token) if punctuation_token.punctuation_type == PunctuationType::Colon) {
                    return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ':'".into()));
                }

                self.step();

                let alternative = self.expression()?;

                let location = Range::from_ranges(vec![condition.location(), question_mark_token.location(), consequent.location(), colon_token.location(), alternative.location()]);
                let ternary_node = TernaryNode::new(location, condition, consequent, alternative);
                let node = Node::Ternary(ternary_node);

                Ok(Box::from(node))
            }
        }
    }
}