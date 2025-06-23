use crate::ast::{AssignmentNode, BinaryOperationNode, BinaryOperationType, FileNode, FunctionNode, IdentifierNode, Node, NumberNode, ScopeNode};
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::token::{KeywordType, OperatorArithmeticType, OperatorAssignmentType, OperatorCategory, ParenthesesState, ParenthesesType, Token};

pub struct Parser {
    index: usize,
    current_token: Option<Token>,
    previous_token: Option<Token>,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            current_token: tokens.get(0).cloned(),
            previous_token: None,
            tokens
        }
    }
    
    fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.index + 1).cloned()
    }
    
    fn step(&mut self) {
        self.previous_token = self.current_token.clone();
        self.index += 1;
        self.current_token = self.tokens.get(self.index).cloned();
    }
    
    fn take(&mut self) -> Option<Token> {
        self.step();
        self.current_token.clone()
    }

    fn get_next_location_from_last_token_location(&self) -> Location {
        let location_char_index = self.previous_token.clone().unwrap_or(Token::Unknown(Location::new(0, 0), ' ')).location().end + 1;
        Location::new(location_char_index, location_char_index)
    }

    fn get_current_location_from_current_token(&self) -> Location {
        self.current_token.clone().unwrap_or(Token::Unknown(Location::new(0, 0), ' ')).location().clone()
    }
    
    pub fn parse(mut self) -> Result<Box<Node>, AxiomError> {
        self.file()
    }

    fn file(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut functions = vec![];
        let mut file_locations = vec![];

        loop {
            let token = match self.current_token.clone() {
                Some(token) => token,
                None => break,
            };

            match token {
                Token::Keyword(keyword_location, keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Function) => {
                    self.step();
                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                    match token {
                        Token::Identifier(identifier_location, identifier_token) => {
                            let identifier_node = IdentifierNode::new(identifier_location.clone(), identifier_token);
                            self.step();
                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                            match token {
                                Token::Parentheses(opening_parentheses_location, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                                    self.step();
                                    let mut parameters = vec![];
                                    let mut locations = vec![keyword_location, identifier_location, opening_parentheses_location];

                                    loop {
                                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                                        if let Token::Parentheses(closing_parentheses_location, parentheses_token) = token {
                                            if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                                                locations.push(closing_parentheses_location);
                                                self.step();
                                                break
                                            }
                                        }

                                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                                        match token {
                                            Token::Identifier(identifier_location, identifier_token) => {
                                                self.step();
                                                locations.push(identifier_location.clone());
                                                parameters.push(IdentifierNode::new(identifier_location, identifier_token));
                                            }
                                            _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                                        }
                                    }

                                    let scope = self.scope()?;
                                    let scope_location = scope.location();
                                    locations.push(scope_location.clone());
                                    let location = Location::from_locations(locations);
                                    file_locations.push(location.clone());
                                    functions.push(Box::from(Node::Function(location, Box::from(FunctionNode::new(identifier_node, parameters, *scope)))));
                                }
                                _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '('".into()))
                            }
                        }
                        _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                    }
                }
                _ => return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
            }
        }

        Ok(Box::from(Node::File(Location::from_locations(file_locations), FileNode::new(functions))))
    }

    fn scope(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;
        
        match token {
            Token::Parentheses(opening_parentheses_location, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Opening)) => {
                self.step();
                let mut statements = vec![];
                let mut locations = vec![opening_parentheses_location];

                loop {
                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                    if let Token::Parentheses(closing_parentheses_location, parentheses_token) = token {
                        if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Closing)) {
                            locations.push(closing_parentheses_location);
                            self.step();
                            break
                        }
                    }

                    let statement = self.statement()?;
                    locations.push(statement.location().clone());
                    statements.push(*statement);
                }

                Ok(Box::from(Node::Scope(Location::from_locations(locations), ScopeNode::new(statements))))
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '{'".into()))
        }
    }

    fn statement(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;
                
        match token {
            Token::Keyword(keyword_location, keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Let) => {
                self.step();
                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                match token {
                    Token::Identifier(identifier_location, identifier_token) => {
                        let identifier_node = IdentifierNode::new(identifier_location.clone(), identifier_token);
                        self.step();
                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                        match token {
                            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                                self.step();
                                let expression = self.expression()?;
                                let expression_location = expression.location();
                                Ok(Box::from(Node::Assignment(Location::from_locations(vec![identifier_location, operator_location, expression_location.clone()]), Box::from(AssignmentNode::new(identifier_node, *expression)))))
                            }
                            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '='".into()))
                        }
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }

    fn expression(&mut self) -> Result<Box<Node>, AxiomError> {
        self.additive()
    }
    
    fn additive(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.multiplicative()?;
        let left_location = &left.location().clone();
                
        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            match token {
                Token::Operator(operator_location, operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Addition) => {
                            self.step();
                            let right = self.multiplicative()?;
                            let right_location = right.location();
                            left = Box::from(Node::BinaryOperation(Location::from_locations(vec![left_location.clone(), operator_location, right_location.clone()]), Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Addition()))))
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction) => {
                            self.step();
                            let right = self.multiplicative()?;
                            let right_location = right.location();
                            left = Box::from(Node::BinaryOperation(Location::from_locations(vec![left_location.clone(), operator_location, right_location.clone()]), Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Subtraction()))))
                        }
                        _ => break
                    }
                }
                _ => break
            }
        }
        
        Ok(left)
    }
    
    fn multiplicative(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.primary()?;
        let left_location = &left.location().clone();
        
        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            match token {
                Token::Operator(operator_location, operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication) => {
                            self.step();
                            let right = self.primary()?;
                            let right_location = right.location();
                            left = Box::from(Node::BinaryOperation(Location::from_locations(vec![left_location.clone(), operator_location, right_location.clone()]), Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Multiplication()))))
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Division) => {
                            self.step();
                            let right = self.primary()?;
                            let right_location = right.location();
                            left = Box::from(Node::BinaryOperation(Location::from_locations(vec![left_location.clone(), operator_location, right_location.clone()]), Box::from(BinaryOperationNode::new(*left, *right, BinaryOperationType::Division()))))
                        }
                        _ => break
                    }
                }
                _ => break
            }
        }
        
        Ok(left)
    }
    
    fn primary(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

        match token {
            Token::Number(number_location, number_token) => {
                self.step();
                Ok(Box::from(Node::Number(number_location, NumberNode::new(number_token))))
            }
            Token::Identifier(identifier_location, identifier_token) => {
                self.step();
                Ok(Box::from(Node::Identifier(identifier_location.clone(), IdentifierNode::new(identifier_location, identifier_token))))
            }
            Token::Parentheses(opening_parentheses_location, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                self.step();
                let expression = self.expression()?;

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                match token {
                    Token::Parentheses(closing_parentheses_location, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) => {
                        self.step();
                        Ok(expression)
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ')'".into()))
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }
}