use crate::analyzer::Analyzer;
use crate::ast::{ParameterNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location};
use crate::utils::SymbolTable;

impl Analyzer for ParameterNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        match symbol_table.get(&self.type_node.identifier_token.name) {
            Some(data_type) => {
                if let DataType::Type(underlying_type) = data_type {
                    self.identifier_node.data_type = *underlying_type.clone();
                } else {
                    errors.push(AxiomError::NotAType(self.type_node.location(), self.type_node.identifier_token.name.clone()));
                }
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.type_node.location(), self.type_node.identifier_token.name.clone()));
            }
        }
        
        self.type_node.analyze(symbol_table, errors);
    }
}