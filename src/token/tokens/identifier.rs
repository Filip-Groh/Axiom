#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierToken {
    pub name: String,
}

impl IdentifierToken {
    pub fn new(name: String) -> IdentifierToken {
        IdentifierToken {
            name
        }
    }
}