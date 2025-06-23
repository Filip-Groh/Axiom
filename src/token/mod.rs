use crate::error::location::Location;
pub use crate::token::tokens::*;

mod tokens;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Location, NumberToken),
    Identifier(Location, IdentifierToken),
    Keyword(Location, KeywordToken),
    Operator(Location, OperatorToken),
    Parentheses(Location, ParenthesesToken),
    Unknown(Location, char),
}

impl Token {
    pub fn location(&self) -> &Location {
        match self {
            Token::Number(location, _) => location,
            Token::Identifier(location, _) => location,
            Token::Keyword(location, _) => location,
            Token::Operator(location, _) => location,
            Token::Parentheses(location, _) => location,
            Token::Unknown(location, _) => location
        }
    }
}