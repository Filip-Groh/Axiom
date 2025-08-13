use crate::ast::{AssignmentNode, BinaryOperationNode, BinaryOperationType, CallNode, DeclarationNode, FileNode, FunctionNode, IdentifierNode, Node, NumberNode, ParameterNode, ReturnNode, ScopeNode};
use crate::error::{AxiomError};
use crate::error::location::Location;
use crate::token::{KeywordType, OperatorArithmeticType, OperatorAssignmentType, OperatorCategory, OperatorComparisonType, ParenthesesState, ParenthesesType, PunctuationType, Token};

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

                                    let mut first_loop = true;
                                    loop {
                                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                                        if let Token::Parentheses(closing_parentheses_location, parentheses_token) = &token {
                                            if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                                                locations.push(closing_parentheses_location.clone());

                                                self.step();

                                                break
                                            }
                                        }

                                        if !first_loop {
                                            if !matches!(&token, Token::Punctuation(_, punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                                            }

                                            locations.push(token.location().clone());

                                            self.step()
                                        }

                                        let parameter = self.parameter()?;
                                        let parameter_location = parameter.location.clone();

                                        parameters.push(parameter);
                                        locations.push(parameter_location);

                                        first_loop = false;
                                    }

                                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                                    let mut output_type = None;
                                    if let Token::Punctuation(punctuation_location, punctuation_token) = &token {
                                        if matches!(punctuation_token.punctuation_type, PunctuationType::Colon) {
                                            self.step();

                                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                                            if let Token::Identifier(type_identifier_location, type_identifier_token) = token {
                                                let type_identifier_node = IdentifierNode::new(type_identifier_location.clone(), type_identifier_token);

                                                self.step();

                                                locations.push(punctuation_location.clone());
                                                locations.push(type_identifier_location.clone());

                                                output_type = Some(Box::from(type_identifier_node));
                                            } else {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                                            }
                                        }
                                    }

                                    let scope = self.scope()?;
                                    let scope_location = scope.location.clone();
                                    locations.push(scope_location.clone());

                                    let location = Location::from_locations(locations);
                                    let function_node = FunctionNode::new(location.clone(), Box::from(identifier_node), parameters, output_type, scope);

                                    file_locations.push(location.clone());
                                    functions.push(Box::from(function_node));
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

        let location = Location::from_locations(file_locations);
        let file_node = FileNode::new(location, functions);
        let node = Node::File(file_node);

        Ok(Box::from(node))
    }
    
    fn parameter(&mut self) -> Result<Box<ParameterNode>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

        if let Token::Identifier(identifier_location, identifier_token) = token {
            let identifier_node = IdentifierNode::new(identifier_location.clone(), identifier_token);

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            if !matches!(&token, Token::Punctuation(_, punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Colon)) {
                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ':'".into()))
            }

            let colon_location = token.location().clone();

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            if let Token::Identifier(type_identifier_location, type_identifier_token) = token {
                let type_identifier_node = IdentifierNode::new(type_identifier_location.clone(), type_identifier_token);

                self.step();

                let location = Location::from_locations(vec![identifier_location, colon_location, type_identifier_location]);
                let parameter_node = ParameterNode::new(location, Box::from(identifier_node), Box::from(type_identifier_node));

                Ok(Box::from(parameter_node))
            } else {
                Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
            }
        } else {
            Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
        }
    }

    fn scope(&mut self) -> Result<Box<ScopeNode>, AxiomError> {
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
                    statements.push(statement);
                }

                let location = Location::from_locations(locations);
                let scope_node = ScopeNode::new(location, statements);

                Ok(Box::from(scope_node))
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
                                let expression_location = expression.location().clone();

                                let location = Location::from_locations(vec![keyword_location, identifier_location, operator_location, expression_location.clone()]);
                                let declaration_node = DeclarationNode::new(location, Box::from(identifier_node), expression);
                                let node = Node::Declaration(declaration_node);

                                Ok(Box::from(node))
                            }
                            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '='".into()))
                        }
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                }
            }
            Token::Keyword(keyword_location, keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Return) => {
                self.step();

                let expression = self.expression()?;
                let expression_location = expression.location().clone();

                let location = Location::from_locations(vec![keyword_location, expression_location]);
                let return_node = ReturnNode::new(location, expression);
                let node = Node::Return(return_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_location, identifier_token) => {
                let identifier_node = IdentifierNode::new(identifier_location, identifier_token);

                self.step();

                let assignment = self.assignment(Box::from(identifier_node))?;

                Ok(assignment)
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }

    fn assignment(&mut self, identifier_node: Box<IdentifierNode>) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

        match token {
            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                self.step();

                let expression = self.expression()?;
                let expression_location = expression.location().clone();

                let location = Location::from_locations(vec![identifier_node.location.clone(), operator_location, expression_location.clone()]);
                let assignment_node = AssignmentNode::new(location, Box::from(identifier_node), expression);
                let node = Node::Assignment(assignment_node);

                Ok(Box::from(node))
            }
            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::AdditionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_location, BinaryOperationType::Addition, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::SubtractionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_location, BinaryOperationType::Subtraction, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::MultiplicationAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_location, BinaryOperationType::Multiplication, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_location, operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::DivisionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_location, BinaryOperationType::Division, |_self| {_self.expression()})?)
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '='".into()))
        }
    }

    fn expression(&mut self) -> Result<Box<Node>, AxiomError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.additive()?;

        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            match token {
                Token::Operator(operator_location, operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Comparison(OperatorComparisonType::Equal) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::Equal, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::NotEqual) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::NotEqual, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThan) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::GreaterThan, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThanOrEqual) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::GreaterThanOrEqual, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThan) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::LessThan, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThanOrEqual) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::LessThanOrEqual, |_self| {_self.additive()})?;
                        }
                        _ => break
                    }
                }
                _ => break
            }
        }

        Ok(left)
    }
    
    fn additive(&mut self) -> Result<Box<Node>, AxiomError> {
        let mut left = self.multiplicative()?;
                
        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            match token {
                Token::Operator(operator_location, operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Addition) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::Addition, |_self| {_self.multiplicative()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::Subtraction, |_self| {_self.multiplicative()})?;
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
        
        loop {
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

            match token {
                Token::Operator(operator_location, operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::Multiplication, |_self| {_self.primary()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Division) => {
                            left = self.binary_operation(left, operator_location, BinaryOperationType::Division, |_self| {_self.primary()})?;
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

                let number_node = NumberNode::new(number_location, number_token);
                let node = Node::Number(number_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_location, identifier_token) => {
                self.step();

                let identifier_node = IdentifierNode::new(identifier_location.clone(), identifier_token);

                if let Some(token) = self.current_token.clone() && matches!(&token, Token::Parentheses(_, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening))) {
                    self.step();

                    let mut parameters = vec![];
                    let mut parameter_locations = vec![identifier_location, token.location().clone(), token.location().clone()];

                    let mut first_loop = true;
                    loop {
                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                        if let Token::Parentheses(parentheses_location, parentheses_token) = &token && matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                            parameter_locations.push(parentheses_location.clone());

                            self.step();

                            break
                        }

                        if !first_loop {
                            if !matches!(&token, Token::Punctuation(_, punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                            }

                            parameter_locations.push(token.location().clone());

                            self.step()
                        }

                        let expression = self.expression()?;
                        let expression_location = expression.location().clone();

                        parameters.push(expression);
                        parameter_locations.push(expression_location);

                        first_loop = false;
                    }

                    let location = Location::from_locations(parameter_locations);
                    let call_node = CallNode::new(location, Box::from(identifier_node), parameters);
                    let node = Node::Call(call_node);

                    return Ok(Box::from(node))
                }

                let node = Node::Identifier(identifier_node);

                Ok(Box::from(node))
            }
            Token::Parentheses(_, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                self.step();

                let expression = self.expression()?;

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_location_from_last_token_location()))?;

                match token {
                    Token::Parentheses(_, parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) => {
                        self.step();

                        Ok(expression)
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ')'".into()))
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }

    fn binary_operation<F>(&mut self, left: Box<Node>, operator_location: Location, binary_operation_type: BinaryOperationType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();
        
        let right = right_fn(self)?;
        let right_location = right.location().clone();

        let location = Location::from_locations(vec![left.location().clone(), operator_location, right_location]);
        let binary_operation_node = BinaryOperationNode::new(location, left, right, binary_operation_type);
        let node = Node::BinaryOperation(binary_operation_node);

        Ok(Box::from(node))
    }

    fn assignment_operation<F>(&mut self, identifier_node: Box<IdentifierNode>, operator_location: Location, binary_operation_type: BinaryOperationType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();

        let right = right_fn(self)?;
        let right_location = right.location().clone();

        let location = Location::from_locations(vec![identifier_node.location.clone(), operator_location.clone(), right_location.clone()]);
        let expression_node = BinaryOperationNode::new(location.clone(), Box::from(Node::Identifier(*identifier_node.clone())), right, binary_operation_type);

        let assignment_node = AssignmentNode::new(location, identifier_node, Box::from(Node::BinaryOperation(expression_node)));
        let node = Node::Assignment(assignment_node);

        Ok(Box::from(node))
    }
}