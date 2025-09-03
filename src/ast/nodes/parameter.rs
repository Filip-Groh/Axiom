use inkwell::types::BasicMetadataTypeEnum;
use crate::analyzer::Analyzer;
use crate::ast::{IdentifierNode, NumberNode};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::utils::SymbolTable;

pub struct ParameterNode {
    location: Range,
    pub identifier_node: Box<IdentifierNode>,
    pub type_node: Box<IdentifierNode>,
}

impl ParameterNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, type_node: Box<IdentifierNode>) -> ParameterNode {
        ParameterNode {
            location,
            identifier_node,
            type_node,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- {}: {}", " ".repeat(indent * 4), self.identifier_node.identifier_token.name, self.type_node.identifier_token.name);
    }
}

impl Location for ParameterNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

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
    }
}

impl CodeGen for ParameterNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let function_context = code_generator.current_function_context.clone().unwrap();
        let parameter_count = function_context.function_type.count_param_types();

        for i in 0..parameter_count {
            let parameter_name = &function_context.parameter_names[i as usize];
            let parameter_type = function_context.function_type.get_param_types()[i as usize];

            let llvm_param = function_context.function_value.get_nth_param(i).unwrap();
            llvm_param.set_name(parameter_name);
            
            let pointer = match parameter_type {
                BasicMetadataTypeEnum::IntType(int_type) => code_generator.builder.build_alloca(int_type, parameter_name).unwrap(),
                _ => unreachable!(),
            };

            code_generator.builder.build_store(pointer, llvm_param).unwrap();

            code_generator.variables.add(parameter_name.clone(), pointer);
        }
    }
}