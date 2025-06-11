pub use crate::token::location::Location;
pub use crate::token::tokens::NumberToken;

mod location;
mod tokens;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(Location, NumberToken),
    Unknown(Location, char),
}