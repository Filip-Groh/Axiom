use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorCategory, OperatorLogicalType, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn conditional_or(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.conditional_and()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Logical(OperatorLogicalType::Or) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Or, |_self| {_self.conditional_and()})?;
                        }
                        _ => break
                    }
                }
                _ => break
            }
        }

        Ok(left)
    }
}