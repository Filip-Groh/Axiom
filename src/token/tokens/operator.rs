#[derive(Debug, PartialEq, Clone)]
pub enum OperatorCategory {
    Arithmetic(OperatorArithmeticType),
    // Bitwise(),
    Comparison(OperatorComparisonType),
    // Logical(),
    Assignment(OperatorAssignmentType),
    Unary(OperatorUnaryType),
    // Ternary(),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorArithmeticType {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorAssignmentType {
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment
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
pub enum OperatorUnaryType {
    Not
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperatorToken {
    pub operator_type: OperatorCategory,
}

impl OperatorToken {
    pub fn new(operator_type: OperatorCategory) -> OperatorToken {
        OperatorToken {
            operator_type
        }
    }
}