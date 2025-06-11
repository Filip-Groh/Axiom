#[derive(Debug, PartialEq)]
pub struct NumberToken {
    value: String,
}

impl NumberToken {
    pub fn new(value: String) -> NumberToken {
        NumberToken {
            value
        }
    }
}