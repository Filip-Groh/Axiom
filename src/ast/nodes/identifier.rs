use inkwell::values::IntValue;
use crate::analyzer::Analyzer;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::token::{IdentifierToken};
use crate::utils::SymbolTable;

#[derive(Clone)]
pub struct IdentifierNode {
    pub location: Location,
    pub data_type: DataType,
    pub identifier_token: IdentifierToken,
}

impl IdentifierNode {
    pub fn new(location: Location, identifier_token: IdentifierToken) -> IdentifierNode {
        IdentifierNode {
            location,
            data_type: DataType::ToBeInferred,
            identifier_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.identifier_token.name);
    }
}

impl Analyzer for IdentifierNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        match symbol_table.get(&self.identifier_token.name) {
            Some(data_type) => {
                self.data_type = data_type.clone()
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.location.clone(), self.identifier_token.name.clone()));
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