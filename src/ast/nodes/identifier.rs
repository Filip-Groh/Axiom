use inkwell::values::IntValue;
use crate::analyzer::Analyzer;
use crate::ast::Node;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::token::{IdentifierToken};
use crate::utils::SymbolTable;

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub data_type: DataType,
    pub identifier_token: IdentifierToken,
}

impl IdentifierNode {
    pub fn new(identifier_token: IdentifierToken) -> IdentifierNode {
        IdentifierNode {
            data_type: DataType::ToBeInferred,
            identifier_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.identifier_token.name);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        Some(Box::from(Node::Identifier(self.clone())))
    }
}

impl Location for IdentifierNode {
    fn location(&self) -> Range {
        self.identifier_token.location()
    }
}

impl Analyzer for IdentifierNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        match symbol_table.get(&self.identifier_token.name) {
            Some(data_type) => {
                self.data_type = data_type.clone()
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.location(), self.identifier_token.name.clone()));
            }
        }
    }
}

impl CodeGen for IdentifierNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let identifier = code_generator.variables.get(&self.identifier_token.name).unwrap();

        let pointer = code_generator.builder.build_load(*identifier, "load").unwrap();

        code_generator.last_assign = Some(IntValue::try_from(pointer).unwrap());
    }
}