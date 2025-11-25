use crate::ast::ScopeNode;
use crate::error::AxiomError;
use crate::parser::Parser;
use crate::error::location::{Location, Range};
use crate::token::{ParenthesesState, ParenthesesType, Token};

impl Parser {
    pub fn scope(&mut self) -> Result<Box<ScopeNode>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Opening)) => {
                self.step();

                let mut statements = vec![];
                let mut locations = vec![parentheses_token.location()];

                loop {
                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                    if let Token::Parentheses(parentheses_token) = token {
                        if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Closing)) {
                            locations.push(parentheses_token.location());

                            self.step();

                            break
                        }
                    }

                    let statement = self.statement()?;

                    locations.push(statement.location());
                    statements.push(statement);
                }

                let location = Range::from_ranges(locations);
                let scope_node = ScopeNode::new(location, statements);

                Ok(Box::from(scope_node))
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '{'".into()))
        }
    }
}