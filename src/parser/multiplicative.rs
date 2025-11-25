use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorArithmeticType, OperatorCategory, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn multiplicative(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.pre_unary()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Multiplication, |_self| {_self.pre_unary()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Division) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Division, |_self| {_self.pre_unary()})?;
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