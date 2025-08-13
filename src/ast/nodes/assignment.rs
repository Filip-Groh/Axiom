use crate::analyzer::Analyzer;
use crate::ast::{IdentifierNode, Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::utils::SymbolTable;

pub struct AssignmentNode {
    pub location: Location,
    pub identifier_node: Box<IdentifierNode>,
    pub expression: Box<Node>,
}

impl AssignmentNode {
    pub fn new(location: Location, identifier_node: Box<IdentifierNode>, expression: Box<Node>) -> AssignmentNode {
        AssignmentNode {
            location,
            identifier_node,
            expression
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {} = ", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        self.expression.display(indent + 1);
    }
}

impl Analyzer for AssignmentNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();

        match symbol_table.get(&self.identifier_node.identifier_token.name) {
            Some(data_type) => {
                if *expression_data_type != *data_type {
                    errors.push(AxiomError::WrongDataType(self.expression.location().clone(), Box::from(data_type.clone()), Box::from(expression_data_type.clone())))
                }
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.identifier_node.location.clone(), self.identifier_node.identifier_token.name.clone()));
            }
        }
    }
}

impl CodeGen for AssignmentNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);
        
        let expression = code_generator.last_assign.take().unwrap();
        let pointer = code_generator.variables.get(&self.identifier_node.identifier_token.name).unwrap();
        
        code_generator.builder.build_store(*pointer, expression).unwrap();
    }
}