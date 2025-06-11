#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Addition(),
    Subtraction(),
    Multiplication(),
    Division()
}

#[derive(Debug, PartialEq)]
pub struct OperatorToken {
    operator_type: OperatorType,
}

impl OperatorToken {
    pub fn new(operator_type: OperatorType) -> OperatorToken {
        OperatorToken {
            operator_type
        }
    }
}