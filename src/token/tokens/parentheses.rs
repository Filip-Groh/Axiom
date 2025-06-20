#[derive(Debug, PartialEq, Clone)]
pub enum ParenthesesState {
    Opening,
    Closing
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParenthesesType {
    Round(ParenthesesState),
    Curly(ParenthesesState)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParenthesesToken {
    pub parentheses_type: ParenthesesType
}

impl ParenthesesToken {
    pub fn new(parentheses_type: ParenthesesType) -> ParenthesesToken {
        ParenthesesToken {
            parentheses_type
        }
    }
}