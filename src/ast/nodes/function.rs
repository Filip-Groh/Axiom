use inkwell::types::{BasicMetadataTypeEnum};
use crate::analyzer::Analyzer;
use crate::ast::{FileNode, IdentifierNode, ParameterNode, ScopeNode};
use crate::codegen::{CodeGen, CodeGenerator, FunctionContext};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::utils::SymbolTable;

pub struct FunctionNode {
    location: Range,
    pub data_type: DataType,
    pub identifier_node: Box<IdentifierNode>,
    pub parameters: Vec<Box<ParameterNode>>,
    pub type_node: Option<Box<IdentifierNode>>,
    pub scope: Box<ScopeNode>,
}

impl FunctionNode {
    pub fn new(location: Range, identifier_node: Box<IdentifierNode>, parameters: Vec<Box<ParameterNode>>, type_node: Option<Box<IdentifierNode>>, scope: Box<ScopeNode>) -> FunctionNode {
        FunctionNode {
            location,
            data_type: DataType::Function(vec![], Box::from(DataType::None)),
            identifier_node,
            parameters,
            type_node,
            scope
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- function {}(", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        for param in &self.parameters {
            param.display(indent + 1);
        }
        println!("{})", " ".repeat(indent * 4));
        self.scope.display(indent + 1);
    }
}

impl Location for FunctionNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

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
        if let Some(type_node) = &self.type_node {
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
        }

        self.data_type = DataType::Function(parameter_types, Box::from(output_type.clone()));

        symbol_table.add(self.identifier_node.identifier_token.name.clone(), self.data_type.clone());

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

impl CodeGen for FunctionNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let (parameter_types, return_type) = match self.data_type.clone() {
            DataType::Function(parameter_types, return_type) => (parameter_types, return_type),
            _ => unreachable!()
        };

        let parameter_types: Vec<BasicMetadataTypeEnum> = parameter_types.iter().map(|parameter_type| {
            match parameter_type {
                DataType::I32 => BasicMetadataTypeEnum::from(code_generator.context.i32_type()),
                DataType::Bool => BasicMetadataTypeEnum::from(code_generator.context.bool_type()),
                _ => unreachable!()
            }
        }).collect();

        let function_type = match *return_type {
            DataType::None => code_generator.context.void_type().fn_type(&*parameter_types, false),
            DataType::I32 => code_generator.context.i32_type().fn_type(&*parameter_types, false),
            DataType::Bool => code_generator.context.bool_type().fn_type(&*parameter_types, false),
            _ => unreachable!()
        };

        let function = code_generator.module.add_function(self.identifier_node.identifier_token.name.as_str(), function_type.clone(), None);
        code_generator.current_function_context = Some(FunctionContext {
            function_value: function,
            function_type,
            parameter_names: self.parameters.iter().map(|parameter| {parameter.identifier_node.identifier_token.name.clone()}).collect(),
        });

        let basic_block = code_generator.context.append_basic_block(function, "entry");
        code_generator.builder.position_at_end(basic_block);

        code_generator.variables.push();

        for parameter in &mut self.parameters {
            parameter.build(code_generator);
        }

        self.scope.build(code_generator);

        code_generator.variables.pop();
    }
}