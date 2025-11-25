use crate::ast::{BinaryType, Node};
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::token::{OperatorCategory, OperatorComparisonType, Token};
use crate::error::location::{Location};

impl Parser {
    pub fn equality(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.bitwise()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Comparison(OperatorComparisonType::Equal) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::Equal, |_self| {_self.bitwise()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::NotEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::NotEqual, |_self| {_self.bitwise()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThan) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::GreaterThan, |_self| {_self.bitwise()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThanOrEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::GreaterThanOrEqual, |_self| {_self.bitwise()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThan) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::LessThan, |_self| {_self.bitwise()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThanOrEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryType::LessThanOrEqual, |_self| {_self.bitwise()})?;
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