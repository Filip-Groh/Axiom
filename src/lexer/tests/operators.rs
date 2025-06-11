#[cfg(test)]

use crate::lexer::tests::common::test_lexer;
use crate::token::{Location, OperatorToken, OperatorType, Token, NumberToken, IdentifierToken};

#[test]
fn test_lexer_operators_alone() {
    assert!(test_lexer("+", vec![
        Token::Operator(Location::new(0, 0), OperatorToken::new(OperatorType::Addition())),
    ]));
    assert!(test_lexer("-", vec![
        Token::Operator(Location::new(0, 0), OperatorToken::new(OperatorType::Subtraction())),
    ]));
    assert!(test_lexer("*", vec![
        Token::Operator(Location::new(0, 0), OperatorToken::new(OperatorType::Multiplication())),
    ]));
    assert!(test_lexer("/", vec![
        Token::Operator(Location::new(0, 0), OperatorToken::new(OperatorType::Division())),
    ]));
}

#[test]
fn test_lexer_operators_alone_whitespace() {
    assert!(test_lexer("  \n  +  \n", vec![
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Addition())),
    ]));
    assert!(test_lexer("  \n  -  \n", vec![
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Subtraction())),
    ]));
    assert!(test_lexer("  \n  *  \n", vec![
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Multiplication())),
    ]));
    assert!(test_lexer("  \n  /  \n", vec![
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Division())),
    ]));
}

#[test]
fn test_lexer_operators_numbers() {
    assert!(test_lexer("1+2", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("1".to_string())),
        Token::Operator(Location::new(1, 1), OperatorToken::new(OperatorType::Addition())),
        Token::Number(Location::new(2, 2), NumberToken::new("2".to_string())),
    ]));
    assert!(test_lexer("3-4", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("3".to_string())),
        Token::Operator(Location::new(1, 1), OperatorToken::new(OperatorType::Subtraction())),
        Token::Number(Location::new(2, 2), NumberToken::new("4".to_string())),
    ]));
    assert!(test_lexer("5*6", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("5".to_string())),
        Token::Operator(Location::new(1, 1), OperatorToken::new(OperatorType::Multiplication())),
        Token::Number(Location::new(2, 2), NumberToken::new("6".to_string())),
    ]));
    assert!(test_lexer("7/8", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("7".to_string())),
        Token::Operator(Location::new(1, 1), OperatorToken::new(OperatorType::Division())),
        Token::Number(Location::new(2, 2), NumberToken::new("8".to_string())),
    ]));
}

#[test]
fn test_lexer_operators_numbers_whitespace() {
    assert!(test_lexer("1 + 2", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("1".to_string())),
        Token::Operator(Location::new(2, 2), OperatorToken::new(OperatorType::Addition())),
        Token::Number(Location::new(4, 4), NumberToken::new("2".to_string())),
    ]));
    assert!(test_lexer("  3  -  4  ", vec![
        Token::Number(Location::new(2, 2), NumberToken::new("3".to_string())),
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Subtraction())),
        Token::Number(Location::new(8, 8), NumberToken::new("4".to_string())),
    ]));
    assert!(test_lexer("  \n 5  *  \n 6   ", vec![
        Token::Number(Location::new(4, 4), NumberToken::new("5".to_string())),
        Token::Operator(Location::new(7, 7), OperatorToken::new(OperatorType::Multiplication())),
        Token::Number(Location::new(12, 12), NumberToken::new("6".to_string())),
    ]));
    assert!(test_lexer("\n  7 \n/ \n\n8  \n", vec![
        Token::Number(Location::new(3, 3), NumberToken::new("7".to_string())),
        Token::Operator(Location::new(6, 6), OperatorToken::new(OperatorType::Division())),
        Token::Number(Location::new(10, 10), NumberToken::new("8".to_string())),
    ]));
}

#[test]
fn test_lexer_operators_numbers_and_operators() {
    assert!(test_lexer("1+hello", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("1".to_string())),
        Token::Operator(Location::new(1, 1), OperatorToken::new(OperatorType::Addition())),
        Token::Identifier(Location::new(2, 6), IdentifierToken::new("hello".to_string())),
    ]));
    assert!(test_lexer("hello-4", vec![
        Token::Identifier(Location::new(0, 4), IdentifierToken::new("hello".to_string())),
        Token::Operator(Location::new(5, 5), OperatorToken::new(OperatorType::Subtraction())),
        Token::Number(Location::new(6, 6), NumberToken::new("4".to_string())),
    ]));
    assert!(test_lexer("hello5*6", vec![
        Token::Identifier(Location::new(0, 5), IdentifierToken::new("hello5".to_string())),
        Token::Operator(Location::new(6, 6), OperatorToken::new(OperatorType::Multiplication())),
        Token::Number(Location::new(7, 7), NumberToken::new("6".to_string())),
    ]));
    assert!(test_lexer("hello7/hello8", vec![
        Token::Identifier(Location::new(0, 5), IdentifierToken::new("hello7".to_string())),
        Token::Operator(Location::new(6, 6), OperatorToken::new(OperatorType::Division())),
        Token::Identifier(Location::new(7, 12), IdentifierToken::new("hello8".to_string())),
    ]));
}

#[test]
fn test_lexer_operators_numbers_and_operators_whitespace() {
    assert!(test_lexer("1 + hello", vec![
        Token::Number(Location::new(0, 0), NumberToken::new("1".to_string())),
        Token::Operator(Location::new(2, 2), OperatorToken::new(OperatorType::Addition())),
        Token::Identifier(Location::new(4, 8), IdentifierToken::new("hello".to_string())),
    ]));
    assert!(test_lexer("  hello  -  4  ", vec![
        Token::Identifier(Location::new(2, 6), IdentifierToken::new("hello".to_string())),
        Token::Operator(Location::new(9, 9), OperatorToken::new(OperatorType::Subtraction())),
        Token::Number(Location::new(12, 12), NumberToken::new("4".to_string())),
    ]));
    assert!(test_lexer(" \n  hello5 \n*  6 \n\n", vec![
        Token::Identifier(Location::new(4, 9), IdentifierToken::new("hello5".to_string())),
        Token::Operator(Location::new(12, 12), OperatorToken::new(OperatorType::Multiplication())),
        Token::Number(Location::new(15, 15), NumberToken::new("6".to_string())),
    ]));
    assert!(test_lexer("hello7\n/\nhello8", vec![
        Token::Identifier(Location::new(0, 5), IdentifierToken::new("hello7".to_string())),
        Token::Operator(Location::new(7, 7), OperatorToken::new(OperatorType::Division())),
        Token::Identifier(Location::new(9, 14), IdentifierToken::new("hello8".to_string())),
    ]));
}