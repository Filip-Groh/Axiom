use crate::ast::ParameterNode;
use crate::codegen::{CodeGen, CodeGenerator};
use inkwell::types::BasicMetadataTypeEnum;

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