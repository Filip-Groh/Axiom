#[derive(Debug, PartialEq, Clone)]
pub struct NumberToken {
    pub value: String,
}

impl NumberToken {
    pub fn new(value: String) -> NumberToken {
        NumberToken {
            value
        }
    }
}