use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorBitwiseType, OperatorCategory, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn shift(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.additive()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Bitwise(OperatorBitwiseType::ShiftRight) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::ShiftRight, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Bitwise(OperatorBitwiseType::ShiftLeft) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::ShiftLeft, |_self| {_self.additive()})?;
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