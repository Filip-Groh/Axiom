use crate::analyzer::Analyzer;
use crate::ast::{TernaryNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for TernaryNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.condition.analyze(symbol_table, errors);
        self.consequent.analyze(symbol_table, errors);
        self.alternative.analyze(symbol_table, errors);

        let condition_data_type = self.condition.data_type();
        let consequent_data_type = self.consequent.data_type();
        let alternative_data_type = self.alternative.data_type();

        if *condition_data_type != DataType::Bool {
            errors.push(AxiomError::WrongDataType(self.condition.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
        }

        if *alternative_data_type != *consequent_data_type {
            errors.push(AxiomError::WrongDataType(self.alternative.location(), Box::from(consequent_data_type.clone()), Box::from(alternative_data_type.clone())))
        }

        self.data_type = consequent_data_type.clone();
    }
}