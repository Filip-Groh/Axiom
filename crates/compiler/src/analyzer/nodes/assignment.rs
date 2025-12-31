use crate::analyzer::Analyzer;
use crate::ast::{AssignmentNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location};
use crate::utils::SymbolTable;

impl Analyzer for AssignmentNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();

        match symbol_table.get(&self.identifier_node.identifier_token.name) {
            Some(data_type) => {
                if *expression_data_type != *data_type {
                    errors.push(AxiomError::WrongDataType(self.expression.location(), Box::from(data_type.clone()), Box::from(expression_data_type.clone())))
                }
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.identifier_node.location(), self.identifier_node.identifier_token.name.clone()));
            }
        }
    }
}