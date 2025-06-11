#[cfg(test)]

use crate::lexer::tests::common::test_lexer;
use crate::token::{Location, NumberToken, Token};

#[test]
fn test_lexer_numbers_single_digit() {
    for x in 0..=9 {
        assert!(test_lexer(x.to_string().as_str(), vec![
            Token::Number(Location::new(0, 0), NumberToken::new(x.to_string())),
        ]));
    }
}

#[test]
fn test_lexer_numbers_single_digit_whitespace() {
    for x in 0..=9 {
        assert!(test_lexer(("  \n  ".to_owned() + x.to_string().as_str() + "  \n   ").as_str(), vec![
            Token::Number(Location::new(5, 5), NumberToken::new(x.to_string())),
        ]));
    }
}

#[test]
fn test_lexer_numbers_multiple_digit() {
    assert!(test_lexer("21", vec![
        Token::Number(Location::new(0, 1), NumberToken::new("21".to_string())),
    ]));
}

#[test]
fn test_lexer_numbers_multiple_digit_whitespace() {
 assert!(test_lexer(" \n   21    \n   ", vec![
     Token::Number(Location::new(5, 6), NumberToken::new("21".to_string())),
 ]));
}

#[test]
fn test_lexer_numbers_multiple_numbers() {
    assert!(test_lexer("21 45 962 106 0", vec![
        Token::Number(Location::new(0, 1), NumberToken::new("21".to_string())),
        Token::Number(Location::new(3, 4), NumberToken::new("45".to_string())),
        Token::Number(Location::new(6, 8), NumberToken::new("962".to_string())),
        Token::Number(Location::new(10, 12), NumberToken::new("106".to_string())),
        Token::Number(Location::new(14, 14), NumberToken::new("0".to_string())),
    ]));
}

#[test]
fn test_lexer_numbers_multiple_numbers_whitespace() {
    assert!(test_lexer("   \n 21 45\n962 106 0\n  ", vec![
        Token::Number(Location::new(5, 6), NumberToken::new("21".to_string())),
        Token::Number(Location::new(8, 9), NumberToken::new("45".to_string())),
        Token::Number(Location::new(11, 13), NumberToken::new("962".to_string())),
        Token::Number(Location::new(15, 17), NumberToken::new("106".to_string())),
        Token::Number(Location::new(19, 19), NumberToken::new("0".to_string())),
    ]));
}

#[test]
fn test_lexer_numbers_long_number() {
    assert!(test_lexer("123456789123456789123456789", vec![
        Token::Number(Location::new(0, 26), NumberToken::new("123456789123456789123456789".to_string())),
    ]));
}

#[test]
fn test_lexer_numbers_long_number_whitespace() {
 assert!(test_lexer(" \n   123456789123456789123456789    \n   ", vec![
     Token::Number(Location::new(5, 31), NumberToken::new("123456789123456789123456789".to_string())),
 ]));
}