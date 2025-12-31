use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorBitwiseType, OperatorCategory, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn bitwise(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.shift()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Bitwise(OperatorBitwiseType::Or) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::BitwiseOr, |_self| {_self.shift()})?;
                        }
                        OperatorCategory::Bitwise(OperatorBitwiseType::And) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::BitwiseAnd, |_self| {_self.shift()})?;
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