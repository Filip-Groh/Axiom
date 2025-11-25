use crate::ast::Node;
use crate::error::AxiomError;
use crate::parser::Parser;

impl Parser {
    pub fn expression(&mut self) -> Result<Box<Node>, AxiomError> {
        self.ternary()
    }
}