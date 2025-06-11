use crate::lexer::Lexer;
use crate::token::Token;

pub fn vec_equals<T: PartialEq>(left: Vec<T>, right: Vec<T>) -> bool {
    if left.len() != right.len() {
        return false
    }
    
    let mut same = true;
    
    for i in 0..left.len() {
        same &= left[i] == right[i]
    }
    
    same
}

pub fn test_lexer(input: &str, expected: Vec<Token>) -> bool {
    let tokens = Lexer::new(&input.to_string()).parse().unwrap();
    vec_equals(tokens, expected)
}