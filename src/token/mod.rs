pub use crate::token::location::Location;
pub use crate::token::tokens::*;

mod location;
mod tokens;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(Location, NumberToken),
    Identifier(Location, IdentifierToken),
    Operator(Location, OperatorToken),
    Unknown(Location, char),
}