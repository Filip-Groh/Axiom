use std::error::Error;
use crate::error::location::{Position, Range};
use crate::token::{NumberToken, Token, IdentifierToken, OperatorToken, OperatorCategory, OperatorArithmeticType, OperatorAssignmentType, ParenthesesToken, ParenthesesType, ParenthesesState, KeywordToken, OperatorComparisonType, PunctuationToken, PunctuationType, OperatorBitwiseType, OperatorLogicalType};

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
            '+' | '-' | '*' | '/' | '=' | '!' | '>' | '<' | '|' | '&' => true,
            _ => false
        }
    }

    fn parse_operator(&mut self) {
        let current_operator = self.current_char.unwrap();
        let start_position = self.position.clone();
        
        match current_operator {
            '+' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::AdditionAssignment)),
                        '+' => Some(OperatorCategory::Arithmetic(OperatorArithmeticType::Increment)),
                        _ => None
                    }
                }, OperatorCategory::Arithmetic(OperatorArithmeticType::Addition));
            }
            '-' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::SubtractionAssignment)),
                        '-' => Some(OperatorCategory::Arithmetic(OperatorArithmeticType::Decrement)),
                        _ => None
                    }
                }, OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction));
            }
            '*' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::MultiplicationAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication));
            }
            '/' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::DivisionAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Arithmetic(OperatorArithmeticType::Division));
            }
            '=' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Comparison(OperatorComparisonType::Equal)),
                        _ => None
                    }
                }, OperatorCategory::Assignment(OperatorAssignmentType::Assignment));
            }
            '!' => {
                self.parse_double_operator(start_position, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Comparison(OperatorComparisonType::NotEqual)),
                        _ => None
                    }
                }, OperatorCategory::Logical(OperatorLogicalType::Not));
            }
            '>' => {
                self.parse_triple_operator(start_position, |char| {
                    match char {
                        '=' => Some((OperatorCategory::Comparison(OperatorComparisonType::GreaterThanOrEqual), false)),
                        '>' => Some((OperatorCategory::Bitwise(OperatorBitwiseType::ShiftRight), true)),
                        _ => None
                    }
                }, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::ShiftRightAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Comparison(OperatorComparisonType::GreaterThan));
            }
            '<' => {
                self.parse_triple_operator(start_position, |char| {
                    match char {
                        '=' => Some((OperatorCategory::Comparison(OperatorComparisonType::LessThanOrEqual), false)),
                        '<' => Some((OperatorCategory::Bitwise(OperatorBitwiseType::ShiftLeft), true)),
                        _ => None
                    }
                }, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::ShiftLeftAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Comparison(OperatorComparisonType::LessThan));
            }
            '|' => {
                self.parse_triple_operator(start_position, |char| {
                    match char {
                        '=' => Some((OperatorCategory::Assignment(OperatorAssignmentType::BitwiseOrAssignment), false)),
                        '|' => Some((OperatorCategory::Logical(OperatorLogicalType::Or), true)),
                        _ => None
                    }
                }, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::OrAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Bitwise(OperatorBitwiseType::Or));
            }
            '&' => {
                self.parse_triple_operator(start_position, |char| {
                    match char {
                        '=' => Some((OperatorCategory::Assignment(OperatorAssignmentType::BitwiseAndAssignment), false)),
                        '&' => Some((OperatorCategory::Logical(OperatorLogicalType::And), true)),
                        _ => None
                    }
                }, |char| {
                    match char {
                        '=' => Some(OperatorCategory::Assignment(OperatorAssignmentType::AndAssignment)),
                        _ => None
                    }
                }, OperatorCategory::Bitwise(OperatorBitwiseType::And));
            }
            _ => ()
        }
    }
    
    fn parse_double_operator<F>(&mut self, start_position: Position, second_char_selector: F, singe_operator_type: OperatorCategory) where F: Fn(char) -> Option<OperatorCategory> {
        let mut token_type = singe_operator_type;

        if let Some(char) = self.peek() && let Some(double_operator_type) = second_char_selector(char) {
            self.step();
            token_type = double_operator_type;
        }

        let location = Range::new(start_position, self.position.clone());
        self.tokens.push(Token::Operator(OperatorToken::new(token_type, location)))
    }

    fn parse_triple_operator<F1, F2>(&mut self, start_position: Position, second_char_selector: F1, third_char_selector: F2, singe_operator_type: OperatorCategory) where F1: Fn(char) -> Option<(OperatorCategory, bool)>, F2: Fn(char) -> Option<OperatorCategory> {
        let mut token_type = singe_operator_type;

        if let Some(char) = self.peek() && let Some((double_operator_type, allow_third)) = second_char_selector(char) {
            self.step();
            token_type = double_operator_type;
            
            if !allow_third {
                let location = Range::new(start_position, self.position.clone());
                self.tokens.push(Token::Operator(OperatorToken::new(token_type, location)));

                return;
            }
        }

        if let Some(char) = self.peek() && let Some(triple_operator_type) = third_char_selector(char) {
            self.step();
            token_type = triple_operator_type;
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
            ',' | ':' | '?' => true,
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
            '?' => {
                self.tokens.push(Token::Punctuation(PunctuationToken::new(PunctuationType::QuestionMark, self.position.clone())))
            }
            _ => ()
        }
    }
}