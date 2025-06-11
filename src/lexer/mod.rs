mod tests;

use std::error::Error;
use crate::token::{Location, NumberToken, Token, IdentifierToken, OperatorType, OperatorToken};

pub struct Lexer{
    chars: Vec<char>,
    index: usize,
    current_char: Option<char>,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(text: &String) -> Lexer {
        let chars: Vec<char> = text.chars().collect();
        
        Lexer {
            current_char: chars.get(0).copied(),
            index: 0,
            tokens: Vec::new(),
            chars
        }
    }
    
    fn peek(&mut self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }
    
    fn step(&mut self) {
        self.index += 1;
        self.current_char = self.chars.get(self.index).copied();
    }
    
    fn take(&mut self) -> Option<char> {
        self.step();
        self.current_char
    }

    pub fn parse(mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut first_loop = true;
        
        loop {
            let current_char = match if first_loop {first_loop = false; self.current_char} else {self.take()} {
                Some(char) => char,
                None => {
                    break;
                }
            };
            
            match current_char {
                char if char.is_whitespace() => {
                    continue;
                }
                char if char.is_ascii_digit() => self.parse_number(),
                char if char.is_alphabetic() => self.parse_identifier(),
                '+' | '-' | '*' | '/' => self.parse_operator(),
                _ => {
                    let location = Location::new(self.index, self.index);
                    self.tokens.push(Token::Unknown(location, current_char));
                }
            }
        }

        Ok(self.tokens)
    }

    fn parse_number(&mut self) {
        let start_location = self.index;
        let mut current_number = vec![self.current_char.unwrap()];

        loop {
            match self.peek() {
                Some(char) => {
                    match char {
                        x if x.is_ascii_digit() => current_number.push(self.take().unwrap()),
                        _ => break
                    }
                }
                None => break
            }
        }
        
        let location = Location::new(start_location, self.index);
        let number_token = NumberToken::new(String::from_iter(current_number));
        self.tokens.push(Token::Number(location, number_token));
    }
    
    fn parse_identifier(&mut self) {
        let start_location = self.index;
        let mut current_identifier = self.current_char.unwrap().to_string();
    
        loop {
            match self.peek() {
                Some(char) => {
                    match char {
                        x if x.is_alphabetic() || x.is_ascii_digit() => current_identifier.push(self.take().unwrap()),
                        _ => break
                    }
                }
                None => break
            }
        }
        
        let location = Location::new(start_location, self.index);
        let identifier_token = IdentifierToken::new(current_identifier);
        self.tokens.push(Token::Identifier(location, identifier_token));
    }

    fn parse_operator(&mut self) {
        let current_operator = self.current_char.unwrap();
        let location = Location::new(self.index, self.index);
        
        match current_operator {
            '+' => {
                self.tokens.push(Token::Operator(location, OperatorToken::new(OperatorType::Addition())))
            }
            '-' => {
                self.tokens.push(Token::Operator(location, OperatorToken::new(OperatorType::Subtraction())))
            }
            '*' => {
                self.tokens.push(Token::Operator(location, OperatorToken::new(OperatorType::Multiplication())))
            }
            '/' => {
                self.tokens.push(Token::Operator(location, OperatorToken::new(OperatorType::Division())))
            }
            _ => ()
        }
    }
    //
    // fn parse_parentheses(&mut self) {
    //     match self.chars.peek() {
    //         Some(char) => {
    //             match char {
    //                 '(' => {
    //                     self.chars.next();
    //                     self.tokens.push(Token::Parentheses(ParenthesesToken::Opening()))
    //                 }
    //                 ')' => {
    //                     self.chars.next();
    //                     self.tokens.push(Token::Parentheses(ParenthesesToken::Closing()))
    //                 }
    //                 _ => ()
    //             }
    //         }
    //         None => ()
    //     }
    // }
}