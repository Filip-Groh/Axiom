use crate::ast::FunctionNode;
use crate::codegen::{CodeGen, CodeGenerator, FunctionContext};
use crate::datatype::DataType;
use inkwell::types::BasicMetadataTypeEnum;

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

        let entry_block = code_generator.context.append_basic_block(function, "entry");
        code_generator.builder.position_at_end(entry_block);

        code_generator.variables.push();

        for parameter in &mut self.parameters {
            parameter.build(code_generator);
        }

        self.scope.build(code_generator);

        code_generator.variables.pop();
    }
}