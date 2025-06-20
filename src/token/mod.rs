pub use crate::token::location::Location;
pub use crate::token::tokens::*;

mod location;
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