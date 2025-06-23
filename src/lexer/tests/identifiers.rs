use crate::error::location::Location;
#[cfg(test)]

use crate::lexer::tests::common::test_lexer;
use crate::token::{IdentifierToken, NumberToken, Token};

#[test]
fn test_lexer_identifiers_lowercase() {
    assert!(test_lexer("hello", vec![
        Token::Identifier(Location::new(0, 4), IdentifierToken::new("hello".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_lowercase_whitespace() {
    assert!(test_lexer("   \n hello   \n ", vec![
        Token::Identifier(Location::new(5, 9), IdentifierToken::new("hello".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_uppercase() {
    assert!(test_lexer("HELLO", vec![
        Token::Identifier(Location::new(0, 4), IdentifierToken::new("HELLO".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_uppercase_whitespace() {
    assert!(test_lexer("   \n HELLO   \n ", vec![
        Token::Identifier(Location::new(5, 9), IdentifierToken::new("HELLO".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_mix() {
    assert!(test_lexer("HelLO", vec![
        Token::Identifier(Location::new(0, 4), IdentifierToken::new("HelLO".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_mix_whitespace() {
    assert!(test_lexer("   \n hELlo   \n ", vec![
        Token::Identifier(Location::new(5, 9), IdentifierToken::new("hELlo".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_number_end() {
    assert!(test_lexer("HelLO2", vec![
        Token::Identifier(Location::new(0, 5), IdentifierToken::new("HelLO2".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_number_end_whitespace() {
    assert!(test_lexer("   \n hELlo2   \n ", vec![
        Token::Identifier(Location::new(5, 10), IdentifierToken::new("hELlo2".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_number_start() {
    assert!(test_lexer("3HelLO", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("3".to_string())),
        Token::Identifier(Location::new(1, 5), IdentifierToken::new("HelLO".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_number_start_whitespace() {
    assert!(test_lexer("   \n 96hELlo   \n ", vec![
        Token::Number(Location::new(5, 6), NumberToken::new("96".to_string())),
        Token::Identifier(Location::new(7, 11), IdentifierToken::new("hELlo".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_multiple_identifiers() {
    assert!(test_lexer("hello WORLD As ideNtiFier", vec![
        Token::Identifier(Location::new(0, 4), IdentifierToken::new("hello".to_string())),
        Token::Identifier(Location::new(6, 10), IdentifierToken::new("WORLD".to_string())),
        Token::Identifier(Location::new(12, 13), IdentifierToken::new("As".to_string())),
        Token::Identifier(Location::new(15, 24), IdentifierToken::new("ideNtiFier".to_string())),
    ]));
}

#[test]
fn test_lexer_identifiers_multiple_identifiers_whitespace() {
    assert!(test_lexer("   \n hello WORLD\nAs ideNtiFier\n  ", vec![
        Token::Identifier(Location::new(5, 9), IdentifierToken::new("hello".to_string())),
        Token::Identifier(Location::new(11, 15), IdentifierToken::new("WORLD".to_string())),
        Token::Identifier(Location::new(17, 18), IdentifierToken::new("As".to_string())),
        Token::Identifier(Location::new(20, 29), IdentifierToken::new("ideNtiFier".to_string())),
    ]));
}