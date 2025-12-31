use crate::analyzer::Analyzer;
use crate::ast::{ReturnNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for ReturnNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();
        
        match symbol_table.get(&"return".to_string()) {
            Some(function_return_type) => {
                if *expression_data_type != *function_return_type {
                    errors.push(AxiomError::WrongDataType(self.expression.location(), Box::from(function_return_type.clone()), Box::from(expression_data_type.clone())))
                }
            }
            None => errors.push(AxiomError::WrongDataType(self.expression.location(), Box::from(DataType::None), Box::from(expression_data_type.clone())))
        }
    }
}