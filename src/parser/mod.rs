use crate::ast::{AssignmentNode, BinaryOperationNode, BinaryOperationType, CallNode, DeclarationNode, FileNode, FunctionNode, IdentifierNode, Node, NumberNode, ParameterNode, ReturnNode, ScopeNode};
use crate::error::{AxiomError};
use crate::error::location::{Location, Position, Range};
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

    fn get_next_position_from_last_token_location(&self) -> Position {
        let last_token_end_position = self.previous_token.clone().unwrap_or(Token::Unknown(Position::new(0, 0), ' ')).location().end;
        Position {
            line: last_token_end_position.line,
            column: last_token_end_position.column + 1
        }
    }

    fn get_current_location_from_current_token(&self) -> Range {
        self.current_token.clone().unwrap_or(Token::Unknown(Position::new(0, 0), ' ')).location()
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
                Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Function) => {
                    self.step();

                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                    match token {
                        Token::Identifier(identifier_token) => {
                            let identifier_node = IdentifierNode::new(identifier_token);

                            self.step();

                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                            match token {
                                Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                                    self.step();

                                    let mut parameters = vec![];
                                    let mut locations = vec![keyword_token.location(), identifier_node.location(), parentheses_token.location()];

                                    let mut first_loop = true;
                                    loop {
                                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                        if let Token::Parentheses(parentheses_token) = &token {
                                            if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                                                locations.push(parentheses_token.location());

                                                self.step();

                                                break
                                            }
                                        }

                                        if !first_loop {
                                            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                                            }

                                            locations.push(token.location());

                                            self.step()
                                        }

                                        let parameter = self.parameter()?;
                                        let parameter_location = parameter.location();

                                        parameters.push(parameter);
                                        locations.push(parameter_location);

                                        first_loop = false;
                                    }

                                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                    let mut output_type = None;
                                    if let Token::Punctuation(punctuation_token) = &token {
                                        if matches!(punctuation_token.punctuation_type, PunctuationType::Colon) {
                                            self.step();

                                            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                                            if let Token::Identifier(type_identifier_token) = token {
                                                let type_identifier_location = type_identifier_token.location();
                                                let type_identifier_node = IdentifierNode::new(type_identifier_token);

                                                self.step();

                                                locations.push(punctuation_token.location());
                                                locations.push(type_identifier_location);

                                                output_type = Some(Box::from(type_identifier_node));
                                            } else {
                                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected identifier".into()))
                                            }
                                        }
                                    }

                                    let scope = self.scope()?;
                                    let scope_location = scope.location();
                                    locations.push(scope_location.clone());

                                    let location = Range::from_ranges(locations);
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

        let location = Range::from_ranges(file_locations);
        let file_node = FileNode::new(location, functions);
        let node = Node::File(file_node);

        Ok(Box::from(node))
    }
    
    fn parameter(&mut self) -> Result<Box<ParameterNode>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        if let Token::Identifier(identifier_token) = token {
            let identifier_node = IdentifierNode::new(identifier_token);

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Colon)) {
                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ':'".into()))
            }

            let colon_location = token.location();

            self.step();

            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            if let Token::Identifier(type_identifier_token) = token {
                let type_identifier_node = IdentifierNode::new(type_identifier_token);

                self.step();

                let location = Range::from_ranges(vec![identifier_node.location(), colon_location, type_identifier_node.location()]);
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
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;
        
        match token {
            Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Opening)) => {
                self.step();

                let mut statements = vec![];
                let mut locations = vec![parentheses_token.location()];

                loop {
                    let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                    if let Token::Parentheses(parentheses_token) = token {
                        if matches!(parentheses_token.parentheses_type, ParenthesesType::Curly(ParenthesesState::Closing)) {
                            locations.push(parentheses_token.location());

                            self.step();

                            break
                        }
                    }

                    let statement = self.statement()?;

                    locations.push(statement.location());
                    statements.push(statement);
                }

                let location = Range::from_ranges(locations);
                let scope_node = ScopeNode::new(location, statements);

                Ok(Box::from(scope_node))
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected '{'".into()))
        }
    }

    fn statement(&mut self) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;
                
        match token {
            Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Let) => {
                self.step();

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                match token {
                    Token::Identifier(identifier_token) => {
                        let identifier_node = IdentifierNode::new(identifier_token);

                        self.step();

                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                        match token {
                            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                                self.step();

                                let expression = self.expression()?;
                                let expression_location = expression.location();

                                let location = Range::from_ranges(vec![keyword_token.location(), identifier_node.location(), operator_token.location(), expression_location.clone()]);
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
            Token::Keyword(keyword_token) if matches!(keyword_token.keyword_type, KeywordType::Return) => {
                self.step();

                let expression = self.expression()?;
                let expression_location = expression.location();

                let location = Range::from_ranges(vec![keyword_token.location(), expression_location]);
                let return_node = ReturnNode::new(location, expression);
                let node = Node::Return(return_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_token) => {
                let identifier_node = IdentifierNode::new(identifier_token);

                self.step();

                let assignment = self.assignment(Box::from(identifier_node))?;

                Ok(assignment)
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }

    fn assignment(&mut self, identifier_node: Box<IdentifierNode>) -> Result<Box<Node>, AxiomError> {
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::Assignment)) => {
                self.step();

                let expression = self.expression()?;
                let expression_location = expression.location();

                let location = Range::from_ranges(vec![identifier_node.location(), operator_token.location(), expression_location.clone()]);
                let assignment_node = AssignmentNode::new(location, Box::from(identifier_node), expression);
                let node = Node::Assignment(assignment_node);

                Ok(Box::from(node))
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::AdditionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryOperationType::Addition, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::SubtractionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryOperationType::Subtraction, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::MultiplicationAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryOperationType::Multiplication, |_self| {_self.expression()})?)
            }
            Token::Operator(operator_token) if matches!(operator_token.operator_type, OperatorCategory::Assignment(OperatorAssignmentType::DivisionAssignment)) => {
                Ok(self.assignment_operation(identifier_node, operator_token.location(), BinaryOperationType::Division, |_self| {_self.expression()})?)
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
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Comparison(OperatorComparisonType::Equal) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::Equal, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::NotEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::NotEqual, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThan) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::GreaterThan, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::GreaterThanOrEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::GreaterThanOrEqual, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThan) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::LessThan, |_self| {_self.additive()})?;
                        }
                        OperatorCategory::Comparison(OperatorComparisonType::LessThanOrEqual) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::LessThanOrEqual, |_self| {_self.additive()})?;
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
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Addition) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::Addition, |_self| {_self.multiplicative()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Subtraction) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::Subtraction, |_self| {_self.multiplicative()})?;
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
            let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

            match token {
                Token::Operator(operator_token) => {
                    match operator_token.operator_type {
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Multiplication) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::Multiplication, |_self| {_self.primary()})?;
                        }
                        OperatorCategory::Arithmetic(OperatorArithmeticType::Division) => {
                            left = self.binary_operation(left, operator_token.location(), BinaryOperationType::Division, |_self| {_self.primary()})?;
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
        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

        match token {
            Token::Number(number_token) => {
                self.step();

                let number_node = NumberNode::new(number_token.location(), number_token);
                let node = Node::Number(number_node);

                Ok(Box::from(node))
            }
            Token::Identifier(identifier_token) => {
                self.step();

                let identifier_node = IdentifierNode::new(identifier_token);

                if let Some(token) = self.current_token.clone() && matches!(&token, Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening))) {
                    self.step();

                    let mut parameters = vec![];
                    let mut parameter_locations = vec![identifier_node.location(), token.location()];

                    let mut first_loop = true;
                    loop {
                        let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                        if let Token::Parentheses(parentheses_token) = &token && matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) {
                            parameter_locations.push(parentheses_token.location());

                            self.step();

                            break
                        }

                        if !first_loop {
                            if !matches!(&token, Token::Punctuation(punctuation_token) if matches!(punctuation_token.punctuation_type, PunctuationType::Comma)) {
                                return Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ',' or ')'".into()))
                            }

                            parameter_locations.push(token.location());

                            self.step()
                        }

                        let expression = self.expression()?;
                        let expression_location = expression.location();

                        parameters.push(expression);
                        parameter_locations.push(expression_location);

                        first_loop = false;
                    }

                    let location = Range::from_ranges(parameter_locations);
                    let call_node = CallNode::new(location, Box::from(identifier_node), parameters);
                    let node = Node::Call(call_node);

                    return Ok(Box::from(node))
                }

                let node = Node::Identifier(identifier_node);

                Ok(Box::from(node))
            }
            Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Opening)) => {
                self.step();

                let expression = self.expression()?;

                let token = self.current_token.clone().ok_or(AxiomError::UnexpectedEOF(self.get_next_position_from_last_token_location()))?;

                match token {
                    Token::Parentheses(parentheses_token) if matches!(parentheses_token.parentheses_type, ParenthesesType::Round(ParenthesesState::Closing)) => {
                        self.step();

                        Ok(expression)
                    }
                    _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Expected ')'".into()))
                }
            }
            _ => Err(AxiomError::SyntaxError(self.get_current_location_from_current_token(), "Unexpected token".into()))
        }
    }

    fn binary_operation<F>(&mut self, left: Box<Node>, operator_location: Range, binary_operation_type: BinaryOperationType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();
        
        let right = right_fn(self)?;
        let right_location = right.location();

        let location = Range::from_ranges(vec![left.location(), operator_location, right_location]);
        let binary_operation_node = BinaryOperationNode::new(location, left, right, binary_operation_type);
        let node = Node::BinaryOperation(binary_operation_node);

        Ok(Box::from(node))
    }

    fn assignment_operation<F>(&mut self, identifier_node: Box<IdentifierNode>, operator_location: Range, binary_operation_type: BinaryOperationType, right_fn: F) -> Result<Box<Node>, AxiomError> where F: Fn(&mut Self) -> Result<Box<Node>, AxiomError> {
        self.step();

        let right = right_fn(self)?;
        let right_location = right.location();

        let location = Range::from_ranges(vec![identifier_node.location(), operator_location.clone(), right_location.clone()]);
        let expression_node = BinaryOperationNode::new(location.clone(), Box::from(Node::Identifier(*identifier_node.clone())), right, binary_operation_type);

        let assignment_node = AssignmentNode::new(location, identifier_node, Box::from(Node::BinaryOperation(expression_node)));
        let node = Node::Assignment(assignment_node);

        Ok(Box::from(node))
    }
}