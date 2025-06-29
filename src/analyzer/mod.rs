mod symbol_table;

use crate::analyzer::symbol_table::SymbolTable;
use crate::ast::Node;
use crate::error::AxiomError;

pub struct Analyzer {
    symbol_table: SymbolTable,
    errors: Vec<AxiomError>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            symbol_table: SymbolTable::new(),
            errors: vec![],
        }    
    }
    
    pub fn analyze(&mut self, node: &Node) -> &Vec<AxiomError> {
        match node {
            Node::File(location, file_node) => {
                for function in &file_node.functions {
                    self.analyze(function);
                }
            }
            Node::Function(location, function_node) => {
                let already_exist = self.symbol_table.has(&function_node.identifier_node.identifier_token.name);
                
                if already_exist {
                    self.errors.push(AxiomError::DuplicatedIdentifier(function_node.identifier_node.location.clone(), function_node.identifier_node.identifier_token.name.clone()));
                }
                
                self.symbol_table.push();
                
                for parameter in &function_node.parameters {
                    let already_exist = self.symbol_table.has(&parameter.identifier_token.name);
                    
                    if already_exist {
                        self.errors.push(AxiomError::DuplicatedIdentifier(parameter.location.clone(), parameter.identifier_token.name.clone()));
                    } else {
                        self.symbol_table.add(parameter.identifier_token.name.clone());
                    }
                }
                
                self.analyze(&function_node.scope);
                
                self.symbol_table.pop();
                
            }
            Node::Scope(location, scope_node) => {
                self.symbol_table.push();
                
                for statement in &scope_node.statements {
                    self.analyze(statement);
                }
                
                self.symbol_table.pop();
            }
            Node::Assignment(location, assignment_node) => {
                self.analyze(&assignment_node.expression);
                
                self.symbol_table.add(assignment_node.identifier_node.identifier_token.name.clone());
            }
            Node::Identifier(location, identifier_node) => {
                match self.symbol_table.get(&identifier_node.identifier_token.name) {
                    Some(_) => {
                        
                    }
                    None => {
                        self.errors.push(AxiomError::IdentifierUsedBeforeDeclaration(identifier_node.location.clone(), identifier_node.identifier_token.name.clone()));
                    }
                }
            }
            Node::BinaryOperation(location, binary_operation_node) => {
                self.analyze(&binary_operation_node.left);
                self.analyze(&binary_operation_node.right);
            }
            Node::Return(location, return_node) => {
                self.analyze(&return_node.expression);
            }
            Node::Number(location, number_node) => {}
        }
        
        &self.errors
    }
}