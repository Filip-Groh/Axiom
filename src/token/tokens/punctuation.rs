#[derive(Debug, PartialEq, Clone)]
pub enum PunctuationType {
    Comma,
    Colon
}

#[derive(Debug, PartialEq, Clone)]
pub struct PunctuationToken {
    pub punctuation_type: PunctuationType
}

impl PunctuationToken {
    pub fn new(punctuation_type: PunctuationType) -> PunctuationToken {
        PunctuationToken {
            punctuation_type
        }
    }
}