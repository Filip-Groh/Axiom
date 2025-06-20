use std::error::Error;
use crate::ast::{AssignmentNode, BinaryOperationNode, BinaryOperationType, FunctionNode, IdentifierNode, Node, NumberNode, ScopeNode};
use crate::token::{KeywordType, OperatorArithmeticType, OperatorAssignmentType, OperatorCategory, ParenthesesState, ParenthesesType, Token};

pub struct Parser {
    index: usize,
    current_token: Option<Token>,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            current_token: tokens.get(0).cloned(),
            tokens
        }
    }
    
    fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.index + 1).cloned()
    }
    
    fn step(&mut self) {
        self.index += 1;
        self.current_token = self.tokens.get(self.index).cloned();
    }
    
    fn take(&mut self) -> Option<Token> {
        self.step();
        self.current_token.clone()
    }
    
    pub fn parse(mut self) -> Result<Box<Node>, Box<dyn Error>> {
        self.file()
    }

    fn file(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let current_token = self.current_token.clone();
        
        match current_token { 
            Some(token) => {
                match token {
                    Token::Keyword(location, keyword_token) => {
                        match keyword_token.keyword_type {
                            KeywordType::Function => {
                                self.step();
                                let current_token = self.current_token.clone();
                                                                
                                match current_token {
                                    Some(token) => {
                                        match token {
                                            Token::Identifier(location, identifier_token) => {
                                                let identifier_node = IdentifierNode::new(location, identifier_token);
                                                
                                                self.step();
                                                let current_token = self.current_token.clone();
                                                        
                                                match current_token { 
                                                    Some(token) => {
                                                        match token {
                                                            Token::Parentheses(location, parentheses_token) => {
                                                                match parentheses_token.parentheses_type {
                                                                    ParenthesesType::Round(state) => {
                                                                        match state {
                                                                            ParenthesesState::Opening => {
                                                                                self.step();
                                                                                
                                                                                let mut parameters = vec![];
                                                                                                                        
                                                                                loop {
                                                                                    let current_token = self.current_token.clone();
                                                                                    
                                                                                    match current_token {
                                                                                        Some(token) => {
                                                                                            match token {
                                                                                                Token::Parentheses(location, parentheses_token) => {
                                                                                                    match parentheses_token.parentheses_type {
                                                                                                        ParenthesesType::Round(state) => {
                                                                                                            match state {
                                                                                                                ParenthesesState::Closing => break,
                                                                                                                _ => {}
                                                                                                            }
                                                                                                        } 
                                                                                                        _ => {}
                                                                                                    }
                                                                                                }
                                                                                                _ => {}
                                                                                            }
                                                                                        }
                                                                                        None => break
                                                                                    }
                                                                                    
                                                                                    let current_token = self.current_token.clone();
                                                                                                                    
                                                                                    match current_token {
                                                                                        Some(token) => {
                                                                                            match token {
                                                                                                Token::Identifier(location, identifier_token) => {
                                                                                                    parameters.push(IdentifierNode::new(location, identifier_token));
                                                                                                    self.step();
                                                                                                }
                                                                                                _ => return Err("SyntaxError".into())
                                                                                            }
                                                                                        }
                                                                                        None => return Err("SyntaxError".into())
                                                                                    }
                                                                                }
                                                                                
                                                                                let current_token = self.current_token.clone();
                                                                                
                                                                                match current_token { 
                                                                                    Some(token) => {
                                                                                        match token {
                                                                                            Token::Parentheses(location, parentheses_token) => {
                                                                                                match parentheses_token.parentheses_type {
                                                                                                    ParenthesesType::Round(state) => {
                                                                                                        match state {
                                                                                                            ParenthesesState::Closing => {
                                                                                                                self.step();
                                                                                                                let scope = self.scope()?;
                                                                                                                Ok(Box::from(Node::Function(Box::from(FunctionNode::new(identifier_node, parameters, *scope)))))
                                                                                                            }
                                                                                                            _ => Err("SyntaxError".into())
                                                                                                        }
                                                                                                    }
                                                                                                    _ => Err("SyntaxError".into())
                                                                                                }
                                                                                            }
                                                                                            _ => Err("SyntaxError".into())
                                                                                        }
                                                                                    }
                                                                                    None => Err("SyntaxError".into())
                                                                                }
                                                                            }
                                                                            _ => Err("SyntaxError".into())
                                                                        }
                                                                    }
                                                                    _ => Err("SyntaxError".into())
                                                                }
                                                            }
                                                            _ => Err("SyntaxError".into())
                                                        }
                                                    }
                                                    None => Err("SyntaxError".into())
                                                }
                                            }
                                            _ => Err("SyntaxError".into())
                                        }
                                    }
                                    None => Err("SyntaxError".into())
                                }
                            }
                            _ => Err("SyntaxError".into())
                        }
                    }
                    _ => Err("SyntaxError".into())
                }
            }
            None => Err("SyntaxError".into())
        }
    }

    fn scope(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let current_token = self.current_token.clone();
        
        match current_token { 
            Some(token) => {
                match token {
                    Token::Parentheses(location, parentheses_token) => {
                        match parentheses_token.parentheses_type {
                            ParenthesesType::Curly(state) => {
                                match state {
                                    ParenthesesState::Opening => {
                                        self.step();
                                        let mut statements = vec![];
                                        
                                        loop {
                                            let current_token = self.current_token.clone();
                                            
                                            match current_token {
                                                Some(token) => {
                                                    match token {
                                                        Token::Parentheses(location, parentheses_token) => {
                                                            match parentheses_token.parentheses_type {
                                                                ParenthesesType::Curly(state) => {
                                                                    match state {
                                                                        ParenthesesState::Closing => break,
                                                                        _ => {}
                                                                    }
                                                                } 
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                None => break
                                            }
                                            
                                            statements.push(*self.statement()?)
                                        }

                                        let current_token = self.current_token.clone();
                                        
                                        match current_token { 
                                            Some(token) => {
                                                match token {
                                                    Token::Parentheses(location, parentheses_token) => {
                                                        match parentheses_token.parentheses_type {
                                                            ParenthesesType::Curly(state) => {
                                                                match state {
                                                                    ParenthesesState::Closing => {
                                                                        self.step();
                                                                        Ok(Box::from(Node::Scope(ScopeNode::new(statements))))
                                                                    }
                                                                    _ => Err("SyntaxError".into())
                                                                }
                                                            }
                                                            _ => Err("SyntaxError".into())
                                                        }
                                                    }
                                                    _ => Err("SyntaxError".into())
                                                }
                                            }
                                            None => Err("SyntaxError".into())
                                        }
                                    }
                                    _ => Err("SyntaxError".into())
                                }
                            }
                            _ => Err("SyntaxError".into())
                        }
                    }
                    _ => Err("SyntaxError".into())
                }
            }
            None => Err("SyntaxError".into())
        }
    }

    fn statement(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let current_token = self.current_token.clone();
                
        match current_token { 
            Some(token) => {
                match token {
                    Token::Keyword(location, keyword_token) => {
                        match keyword_token.keyword_type {
                            KeywordType::Let => {
                                self.step();
                                let current_token = self.current_token.clone();
                                
                                match current_token {
                                    Some(token) => {
                                        match token {
                                            Token::Identifier(location, identifier_token) => {
                                                let identifier_node = IdentifierNode::new(location, identifier_token);
                                                
                                                self.step();
                                                let current_token = self.current_token.clone();
                                                
                                                match current_token {
                                                    Some(token) => {
                                                        match token {
                                                            Token::Operator(location, operator_token) => {
                                                                match operator_token.operator_type {
                                                                    OperatorCategory::Assignment(assignment_type) => {
                                                                        match assignment_type {
                                                                            OperatorAssignmentType::Assignment => {
                                                                                self.step();
                                                                                let expression = self.expression()?;
                                                                                
                                                                                Ok(Box::from(Node::Assignment(Box::from(AssignmentNode::new(identifier_node, *expression)))))
                                                                            }
                                                                            _ => Err("SyntaxError".into()) 
                                                                        }
                                                                    }
                                                                    _ => Err("SyntaxError".into()) 
                                                                }
                                                            }
                                                            _ => Err("SyntaxError".into()) 
                                                        }
                                                    }
                                                    None => Err("SyntaxError".into())
                                                }
                                            }
                                            _ => Err("SyntaxError".into())
                                        }
                                    }
                                    None => Err("SyntaxError".into())
                                }
                            }
                            _ => Err("SyntaxError".into())
                        }
                    }
                    _ => Err("SyntaxError".into())
                }
            }
            None => Err("SyntaxError".into())
        }
    }

    fn expression(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        self.additive()
    }
    
    fn additive(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let mut left = self.multiplicative()?;
                
        loop {
            let current_token = self.current_token.clone();
            
            match current_token {
                Some(token) => {
                    match token {
                        Token::Operator(location, operator_token) => {
                            match operator_token.operator_type {
                                OperatorCategory::Arithmetic(arithmetic_type) => {
                                    match arithmetic_type {
                                        OperatorArithmeticType::Addition => {
                                            self.step();
                                            let right = self.multiplicative()?;
                                            left = Box::from(Node::BinaryOperation(Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Addition()))))
                                        }
                                        OperatorArithmeticType::Subtraction => {
                                            self.step();
                                            let right = self.multiplicative()?;
                                            left = Box::from(Node::BinaryOperation(Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Subtraction()))))                           
                                        }
                                        _ => break
                                    }
                                }
                                _ => break
                            }
                        }
                        _ => break
                    }
                }
                None => break 
            }
        }
        
        Ok(left)
    }
    
    fn multiplicative(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let mut left = self.primary()?;
        
        loop {
            let current_token = self.current_token.clone();
            
            match current_token {
                Some(token) => {
                    match token {
                        Token::Operator(location, operator_token) => {
                            match operator_token.operator_type {
                                OperatorCategory::Arithmetic(arithmetic_type) => {
                                    match arithmetic_type {
                                        OperatorArithmeticType::Multiplication => {
                                            self.step();
                                            let right = self.primary()?;
                                            left = Box::from(Node::BinaryOperation(Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Multiplication()))))
                                        }
                                        OperatorArithmeticType::Division => {
                                            self.step();
                                            let right = self.primary()?;
                                            left = Box::from(Node::BinaryOperation(Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Division()))))
                                        }
                                        _ => break
                                    }
                                }
                                _ => break
                            }
                        }
                        _ => break
                    }
                }
                None => break 
            }
        }
        
        Ok(left)
    }
    
    fn primary(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let current_token = self.current_token.clone();
        
        match current_token { 
            Some(token) => {
                match token {
                    Token::Number(location, number_token) => {
                        self.step();
                        Ok(Box::from(Node::Number(NumberNode::new(location, number_token))))
                    },
                    Token::Identifier(location, identifier_token) => {
                        self.step();
                        Ok(Box::from(Node::Identifier(IdentifierNode::new(location, identifier_token))))
                    },
                    Token::Parentheses(location, parentheses_token) => {
                        match parentheses_token.parentheses_type {
                            ParenthesesType::Round(state) => {
                                match state {
                                    ParenthesesState::Opening => {
                                        self.step();
                                        let expression = self.expression()?;
                                        
                                        let current_token = self.current_token.clone();
                                        
                                        match current_token { 
                                            Some(token) => {
                                                match token {
                                                    Token::Parentheses(location, parentheses_token) => {
                                                        match parentheses_token.parentheses_type {
                                                            ParenthesesType::Round(state) => {
                                                                match state {
                                                                    ParenthesesState::Closing => {
                                                                        self.step();
                                                                        Ok(expression)
                                                                    }
                                                                    _ => Err("SyntaxError".into())
                                                                }
                                                            }
                                                            _ => Err("SyntaxError".into())
                                                        }
                                                    }
                                                    _ => Err("SyntaxError".into())
                                                }
                                            }
                                            None => Err("SyntaxError".into())
                                        }
                                    }
                                    _ => Err("SyntaxError".into())
                                }
                            }
                            _ => Err("SyntaxError".into())
                        }
                    }
                    _ => Err("SyntaxError".into())
                }
            }
            None => Err("SyntaxError".into())
        }
    }
}