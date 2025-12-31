use crate::error::location::{Location, Position, Range};
pub use crate::token::tokens::*;

mod tokens;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(NumberToken),
    Identifier(IdentifierToken),
    Keyword(KeywordToken),
    Operator(OperatorToken),
    Parentheses(ParenthesesToken),
    Punctuation(PunctuationToken),
    Unknown(Position, char),
}

impl Location for Token {
    fn location(&self) -> Range {
        match self {
            Token::Number(number_token) => number_token.location(),
            Token::Identifier(identifier_token) => identifier_token.location(),
            Token::Keyword(keyword_token) => keyword_token.location(),
            Token::Operator(operator_token) => operator_token.location(),
            Token::Parentheses(parentheses_token) => parentheses_token.location(),
            Token::Punctuation(punctuation_token) => punctuation_token.location(),
            Token::Unknown(position, _) => Range {
                start: position.clone(),
                end: position.clone(),
            }
        }
    }
}