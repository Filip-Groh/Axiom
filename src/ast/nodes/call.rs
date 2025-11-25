use std::cmp::max;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValue};
use crate::analyzer::Analyzer;
use crate::ast::{BinaryNode, IdentifierNode, Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::utils::SymbolTable;

pub struct CallNode {
    location: Range,
    pub data_type: DataType,
    pub identifier_node: Box<IdentifierNode>,
    pub parameters: Vec<Box<Node>>,
}

impl CallNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, parameters: Vec<Box<Node>>) -> CallNode {
        CallNode {
            location,
            data_type: DataType::ToBeInferred,
            identifier_node,
            parameters,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- {}(", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        for param in &self.parameters {
            param.display(indent + 1);
        }
        println!("{})", " ".repeat(indent * 4));
    }
}

impl Location for CallNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

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
    }
}

impl CodeGen for CallNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let function = code_generator.module.get_function(&*self.identifier_node.identifier_token.name).unwrap();

        let parameters: Vec<BasicMetadataValueEnum> = self.parameters.iter_mut().map(|parameter| {
            parameter.build(code_generator);
            code_generator.last_assign.take().unwrap().into()
        }).collect();

        let call = code_generator.builder.build_call(function, &*parameters, "call").unwrap();

        let expression = call.try_as_basic_value().left().unwrap();

        code_generator.last_assign = Some(expression.into_int_value())
    }
}