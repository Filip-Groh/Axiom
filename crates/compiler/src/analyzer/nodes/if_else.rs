use crate::analyzer::Analyzer;
use crate::ast::{IfElseNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for IfElseNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.condition.analyze(symbol_table, errors);
        self.consequent.analyze(symbol_table, errors);

        let condition_data_type = self.condition.data_type();

        if *condition_data_type != DataType::Bool {
            errors.push(AxiomError::WrongDataType(self.condition.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
        }

        for conditional_alternative in &mut self.conditional_alternatives {
            conditional_alternative.0.analyze(symbol_table, errors);
            conditional_alternative.1.analyze(symbol_table, errors);

            let condition_data_type = conditional_alternative.0.data_type();

            if *condition_data_type != DataType::Bool {
                errors.push(AxiomError::WrongDataType(conditional_alternative.0.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
            }
        }

        if let Some(alternative) = &mut self.alternative {
            alternative.analyze(symbol_table, errors);
        }
    }
}