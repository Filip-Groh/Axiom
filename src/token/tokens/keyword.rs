#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
    Let,
    Function
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeywordToken {
    pub keyword_type: KeywordType,
}

impl KeywordToken {
    pub fn new(keyword_type: KeywordType) -> KeywordToken {
        KeywordToken {
            keyword_type
        }
    }
    
    pub fn get_keyword_type(identifier: &String) -> Option<KeywordType> {
        match identifier.as_str() {
            "let" => Some(KeywordType::Let),
            "function" => Some(KeywordType::Function),
            _ => None,
        }
    }
}