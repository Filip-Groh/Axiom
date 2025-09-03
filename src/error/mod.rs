use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::datatype::DataType;
use crate::error::location::{Position, Range};

pub mod location;

#[derive(Debug)]
pub enum AxiomError {
    UnexpectedEOF(Position),
    SyntaxError(Range, String),
    DuplicatedIdentifier(Range, String),
    IdentifierUsedBeforeDeclaration(Range, String),
    WrongDataType(Range, Box<DataType>, Box<DataType>),
    NotAFunction(Range, String),
    MismatchedNumberOfParameters(Range, String, usize, usize),
    NotAType(Range, String),
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