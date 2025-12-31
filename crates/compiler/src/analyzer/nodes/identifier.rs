use crate::analyzer::Analyzer;
use crate::ast::{IdentifierNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location};
use crate::utils::SymbolTable;

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