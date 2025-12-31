use crate::error::location::{Location, Range};

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorCategory {
    Arithmetic(OperatorArithmeticType),
    Bitwise(OperatorBitwiseType),
    Comparison(OperatorComparisonType),
    Logical(OperatorLogicalType),
    Assignment(OperatorAssignmentType),
    // Ternary(),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorArithmeticType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorBitwiseType {
    ShiftLeft,
    ShiftRight,
    Or,
    And
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorLogicalType {
    Not,
    Or,
    And
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorAssignmentType {
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ShiftLeftAssignment,
    ShiftRightAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    AndAssignment,
    OrAssignment,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperatorToken {
    pub operator_type: OperatorCategory,
    location: Range
}

impl OperatorToken {
    pub fn new(operator_type: OperatorCategory, location: Range) -> OperatorToken {
        OperatorToken {
            operator_type,
            location
        }
    }
}

impl Location for OperatorToken {
    fn location(&self) -> Range {
        self.location.clone()
    }
}