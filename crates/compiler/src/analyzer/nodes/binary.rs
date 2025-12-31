use crate::analyzer::Analyzer;
use crate::ast::{BinaryNode, BinaryType};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for BinaryNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.left.analyze(symbol_table, errors);
        self.right.analyze(symbol_table, errors);

        let left_data_type = self.left.data_type();
        let right_data_type = self.right.data_type();

        if *right_data_type != *left_data_type {
            errors.push(AxiomError::WrongDataType(self.right.location(), Box::from(left_data_type.clone()), Box::from(right_data_type.clone())))
        }

        match self.operation_type {
            BinaryType::Addition | BinaryType::Subtraction | BinaryType::Multiplication | BinaryType::Division | BinaryType::ShiftLeft | BinaryType::ShiftRight | BinaryType::BitwiseAnd | BinaryType::BitwiseOr | BinaryType::Or | BinaryType::And => self.data_type = left_data_type.clone(),

            BinaryType::Equal | BinaryType::NotEqual | BinaryType::GreaterThan | BinaryType::LessThan | BinaryType::GreaterThanOrEqual | BinaryType::LessThanOrEqual => self.data_type = DataType::Bool
        }
    }
}