use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorArithmeticType, OperatorCategory, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn additive(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.multiplicative()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Addition) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Addition, |_self| {_self.multiplicative()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Subtraction, |_self| {_self.multiplicative()})?;
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