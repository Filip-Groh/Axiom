#[derive(Debug, PartialEq)]
pub struct IdentifierToken {
    name: String,
}

impl IdentifierToken {
    pub fn new(name: String) -> IdentifierToken {
        IdentifierToken {
            name
        }
    }
}