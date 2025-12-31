use crate::analyzer::Analyzer;
use crate::ast::{FunctionNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location};
use crate::utils::SymbolTable;

impl Analyzer for FunctionNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        let already_exist = symbol_table.has(&self.identifier_node.identifier_token.name);

        if already_exist {
            errors.push(AxiomError::DuplicatedIdentifier(self.identifier_node.location(), self.identifier_node.identifier_token.name.clone()));
        }

        let mut parameter_types = vec![];
        for parameter in &mut self.parameters {
            parameter.analyze(symbol_table, errors);

            parameter_types.push(parameter.identifier_node.data_type.clone());
        }

        let mut output_type = DataType::None;
        if let Some(type_node) = &mut self.type_node {
            match symbol_table.get(&type_node.identifier_token.name) {
                Some(data_type) => {
                    if let DataType::Type(underlying_type) = data_type {
                        output_type = *underlying_type.clone();
                    } else {
                        errors.push(AxiomError::NotAType(type_node.location(), type_node.identifier_token.name.clone()));
                    }
                }
                None => {
                    errors.push(AxiomError::IdentifierUsedBeforeDeclaration(type_node.location(), type_node.identifier_token.name.clone()));
                }
            }

            type_node.analyze(symbol_table, errors);
        }

        self.data_type = DataType::Function(parameter_types, Box::from(output_type.clone()));

        symbol_table.add(self.identifier_node.identifier_token.name.clone(), self.data_type.clone());
        
        self.identifier_node.analyze(symbol_table, errors);

        symbol_table.push();

        symbol_table.add("return".to_string(), output_type);

        for parameter in &self.parameters {
            let already_exist = symbol_table.has(&parameter.identifier_node.identifier_token.name);

            if already_exist {
                errors.push(AxiomError::DuplicatedIdentifier(parameter.location(), parameter.identifier_node.identifier_token.name.clone()));
            } else {
                symbol_table.add(parameter.identifier_node.identifier_token.name.clone(), parameter.identifier_node.data_type.clone());
            }
        }

        self.scope.analyze(symbol_table, errors);

        symbol_table.pop();
    }
}