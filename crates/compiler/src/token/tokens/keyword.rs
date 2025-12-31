use crate::error::location::{Location, Range};

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
    Let,
    Function,
    Return,
    If,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeywordToken {
    pub keyword_type: KeywordType,
    location: Range
}

impl KeywordToken {
    pub fn new(keyword_type: KeywordType, location: Range) -> KeywordToken {
        KeywordToken {
            keyword_type,
            location
        }
    }
    
    pub fn get_keyword_type(identifier: &String) -> Option<KeywordType> {
        match identifier.as_str() {
            "let" => Some(KeywordType::Let),
            "function" => Some(KeywordType::Function),
            "return" => Some(KeywordType::Return),
            "if" => Some(KeywordType::If),
            "else" => Some(KeywordType::Else),
            _ => None,
        }
    }
}

impl Location for KeywordToken {
    fn location(&self) -> Range {
        self.location.clone()
    }
}