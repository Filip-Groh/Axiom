#[derive(Debug, PartialEq, Clone)]
pub enum OperatorCategory {
    Arithmetic(OperatorArithmeticType),
    // Comparison(),
    // Logical(),
    // Bitwise(),
    Assignment(OperatorAssignmentType),
    // Unary(),
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
    Assignment
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