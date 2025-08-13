use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::datatype::DataType;
use crate::error::location::Location;

pub mod location;

#[derive(Debug)]
pub enum AxiomError {
    UnexpectedEOF(Location),
    SyntaxError(Location, String),
    DuplicatedIdentifier(Location, String),
    IdentifierUsedBeforeDeclaration(Location, String),
    WrongDataType(Location, Box<DataType>, Box<DataType>),
    NotAFunction(Location, String),
    MismatchedNumberOfParameters(Location, String, usize, usize),
    NotAType(Location, String),
}

impl Display for AxiomError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AxiomError::UnexpectedEOF(location) => write!(f, "[{:?}] - Unexpected EOF", location),
            AxiomError::SyntaxError(location, message) => write!(f, "[{:?}] - SyntaxError: {}", location, message),
            AxiomError::DuplicatedIdentifier(location, identifier) => write!(f, "[{:?}] - Duplicated identifier: {}", location, identifier),
            AxiomError::IdentifierUsedBeforeDeclaration(location, identifier) => write!(f, "[{:?}] - Identifier used before declaration: {}", location, identifier),
            AxiomError::WrongDataType(location, expected, received) => write!(f, "[{:?}] - Expected DataType: {}, but found: {}", location, expected, received),
            AxiomError::NotAFunction(location, identifier) => write!(f, "[{:?}] - {} is not a function", location, identifier),
            AxiomError::MismatchedNumberOfParameters(location, identifier, function_parameter_count, call_parameter_count) => write!(f, "[{:?}] - Mismatched number of parameters, function {} takes {} parameters, but given {}", location, identifier, function_parameter_count, call_parameter_count),
            AxiomError::NotAType(location, identifier) => write!(f, "[{:?}] - {} is not a type", location, identifier),
        }
    }
}

impl Error for AxiomError {}