mod common;
mod numbers;
mod identifiers;
mod operators;

#[cfg(test)]
mod general {
    use crate::lexer::tests::common::test_lexer;

    #[test]
    fn test_lexer_empty() {
        assert!(test_lexer("", vec![]));
    }
    
    #[test]
    fn test_lexer_whitespace_space() {
        assert!(test_lexer(" ", vec![]));
        assert!(test_lexer("             ", vec![]));
    }
    
    #[test]
    fn test_lexer_whitespace_newline() {
        assert!(test_lexer("\n", vec![]));
        assert!(test_lexer("\n\n\n\n\n\n\n\n\n", vec![]));
    }
    
    #[test]
    fn test_lexer_whitespace_combination() {
        assert!(test_lexer("\n ", vec![]));
        assert!(test_lexer(" \n", vec![]));
        assert!(test_lexer("\n    \n\n\n     \n\n\n    \n\n", vec![]));
        assert!(test_lexer("    \n    \n     \n\n\n    \n\n", vec![]));
    }
}