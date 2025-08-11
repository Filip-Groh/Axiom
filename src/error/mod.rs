use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::error::location::Location;

pub mod location;

#[derive(Debug)]
pub enum AxiomError {
    UnexpectedEOF(Location),
    SyntaxError(Location, String),
    DuplicatedIdentifier(Location, String),
    IdentifierUsedBeforeDeclaration(Location, String),
}

impl Display for AxiomError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AxiomError::UnexpectedEOF(location) => write!(f, "[{:?}] - Unexpected EOF", location),
            AxiomError::SyntaxError(location, message) => write!(f, "[{:?}] - SyntaxError: {}", location, message),
            AxiomError::DuplicatedIdentifier(location, identifier) => write!(f, "[{:?}] - Duplicated identifier: {}", location, identifier),
            AxiomError::IdentifierUsedBeforeDeclaration(location, identifier) => write!(f, "[{:?}] - Identifier used before declaration: {}", location, identifier),
        }
    }
}

impl Error for AxiomError {}