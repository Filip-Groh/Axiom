use std::error::Error;
use crate::error::location::{Position, Range};
use crate::token::{NumberToken, Token, IdentifierToken, OperatorToken, OperatorCategory, OperatorArithmeticType, OperatorAssignmentType, ParenthesesToken, ParenthesesType, ParenthesesState, KeywordToken, OperatorComparisonType, OperatorUnaryType, PunctuationToken, PunctuationType};

pub struct Lexer{
    chars: Vec<char>,
    index: usize,
    position: Position,
    current_char: Option<char>,
    tokens: Vec<Token>,
    is_next_char_on_new_line: bool,
}

impl Lexer {
    pub fn new(text: &String) -> Lexer {
        let chars: Vec<char> = text.chars().collect();
        
        Lexer {
            current_char: chars.get(0).cloned(),
            index: 0,
            position: Position::new(0, 0),
            tokens: Vec::new(),
            chars,
            is_next_char_on_new_line: false,
        }
    }
    
    fn peek(&mut self) -> Option<char> {
        self.chars.get(self.index + 1).cloned()
    }
    
    fn step(&mut self) {
        self.index += 1;
        self.current_char = self.chars.get(self.index).cloned();

        if self.is_next_char_on_new_line {
            self.position.column = 0;
            self.position.line += 1;
        } else {
            self.position.column += 1;
        }

        self.is_next_char_on_new_line = self.current_char == Some('\n');
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
                char if Lexer::is_operator(char) => self.parse_operator(),
                char if Lexer::is_parentheses(char) => self.parse_parentheses(),
                char if Lexer::is_punctuation(char) => self.parse_punctuation(),
                _ => {
                    self.tokens.push(Token::Unknown(self.position.clone(), current_char));
                }
            }
        }

        Ok(self.tokens)
    }

    fn parse_number(&mut self) {
        let start_position = self.position.clone();
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
        
        let location = Range::new(start_position, self.position.clone());
        let number_token = NumberToken::new(String::from_iter(current_number), location);
        self.tokens.push(Token::Number(number_token));
    }
    
    fn parse_identifier(&mut self) {
        let start_position = self.position.clone();
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
        
        let location = Range::new(start_position, self.position.clone());
        let keyword_type = KeywordToken::get_keyword_type(&current_identifier);
        match keyword_type {
            Some(keyword) => {
                self.tokens.push(Token::Keyword(KeywordToken::new(keyword, location)));
            }
            None => {
                self.tokens.push(Token::Identifier(IdentifierToken::new(current_identifier, location)));
            }
        }
    }
    
    fn is_operator(current_char: char) -> bool {
        match current_char {
            '+' | '-' | '*' | '/' | '=' | '!' | '>' | '<' => true,
            _ => false
        }
    }

    fn parse_operator(&mut self) {
        let current_operator = self.current_char.unwrap();
        let start_position = self.position.clone();
        
        match current_operator {
            '+' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Arithmetic(OperatorArithmeticType::Addition), OperatorCategory::Assignment(OperatorAssignmentType::AdditionAssignment));
            }
            '-' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction), OperatorCategory::Assignment(OperatorAssignmentType::SubtractionAssignment));
            }
            '*' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication), OperatorCategory::Assignment(OperatorAssignmentType::MultiplicationAssignment));
            }
            '/' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Arithmetic(OperatorArithmeticType::Division), OperatorCategory::Assignment(OperatorAssignmentType::DivisionAssignment));
            }
            '=' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Assignment(OperatorAssignmentType::Assignment), OperatorCategory::Comparison(OperatorComparisonType::Equal));
            }
            '!' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Unary(OperatorUnaryType::Not), OperatorCategory::Comparison(OperatorComparisonType::NotEqual));
            }
            '>' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Comparison(OperatorComparisonType::GreaterThan), OperatorCategory::Comparison(OperatorComparisonType::GreaterThanOrEqual));
            }
            '<' => {
                self.parse_double_operator(start_position, '=', OperatorCategory::Comparison(OperatorComparisonType::LessThan), OperatorCategory::Comparison(OperatorComparisonType::LessThanOrEqual));
            }
            _ => ()
        }
    }
    
    fn parse_double_operator(&mut self, start_position: Position, second_char: char, singe_operator_type: OperatorCategory, double_operator_type: OperatorCategory) {
        let mut token_type = singe_operator_type;

        if let Some(char) = self.peek() && char == second_char {
            self.step();
            token_type = double_operator_type;
        }

        let location = Range::new(start_position, self.position.clone());
        self.tokens.push(Token::Operator(OperatorToken::new(token_type, location)))
    }
    
    fn is_parentheses(current_char: char) -> bool {
        match current_char {
            '(' | ')' | '{' | '}' => true,
            _ => false
        }
    }
    
    fn parse_parentheses(&mut self) {
        let current_parentheses = self.current_char.unwrap();
        
        match current_parentheses {
            '(' => {
                self.tokens.push(Token::Parentheses(ParenthesesToken::new(ParenthesesType::Round(ParenthesesState::Opening), self.position.clone())));
            }
            ')' => {
                self.tokens.push(Token::Parentheses(ParenthesesToken::new(ParenthesesType::Round(ParenthesesState::Closing), self.position.clone())))
            }
            '{' => {
                self.tokens.push(Token::Parentheses(ParenthesesToken::new(ParenthesesType::Curly(ParenthesesState::Opening), self.position.clone())))
            }
            '}' => {
                self.tokens.push(Token::Parentheses(ParenthesesToken::new(ParenthesesType::Curly(ParenthesesState::Closing), self.position.clone())))
            }
            _ => ()
        }
    }
    
    fn is_punctuation(current_char: char) -> bool {
        match current_char {
            ',' | ':' => true,
            _ => false
        }
    }
    
    fn parse_punctuation(&mut self) {
        let current_punctuation = self.current_char.unwrap();
        
        match current_punctuation {
            ',' => {
                self.tokens.push(Token::Punctuation(PunctuationToken::new(PunctuationType::Comma, self.position.clone())))
            }
            ':' => {
                self.tokens.push(Token::Punctuation(PunctuationToken::new(PunctuationType::Colon, self.position.clone())))
            }
            _ => ()
        }
    }
}