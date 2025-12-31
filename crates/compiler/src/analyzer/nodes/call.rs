use std::cmp::max;
use crate::analyzer::Analyzer;
use crate::ast::{CallNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location};
use crate::utils::SymbolTable;

impl Analyzer for CallNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        let function = symbol_table.get(&self.identifier_node.identifier_token.name);

        match function {
            Some(data_type) => {
                if let DataType::Function(parameter_data_types, output_data_type) = data_type.clone() {
                    for parameter in &mut self.parameters {
                        parameter.analyze(symbol_table, errors);
                    }

                    for i in 0..max(self.parameters.len(), parameter_data_types.len()) {
                        let call_parameter = self.parameters.get(i);
                        let function_parameter = parameter_data_types.get(i);

                        if call_parameter.is_none() || function_parameter.is_none() {
                            errors.push(AxiomError::MismatchedNumberOfParameters(self.location.clone(), self.identifier_node.identifier_token.name.clone(), parameter_data_types.len(), self.parameters.len()));
                            break;
                        }

                        if *call_parameter.unwrap().data_type() != *function_parameter.unwrap() {
                            errors.push(AxiomError::WrongDataType(call_parameter.unwrap().location(), Box::from(function_parameter.unwrap().clone()), Box::from(call_parameter.unwrap().data_type().clone())));
                        }
                    }

                    self.data_type = *output_data_type.clone();
                } else {
                    errors.push(AxiomError::NotAFunction(self.identifier_node.location(), self.identifier_node.identifier_token.name.clone()));
                }
            }
            None => {
                errors.push(AxiomError::IdentifierUsedBeforeDeclaration(self.identifier_node.location(), self.identifier_node.identifier_token.name.clone()));
            }
        }

        self.identifier_node.analyze(symbol_table, errors);
    }
}