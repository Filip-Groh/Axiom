use crate::ast::CallNode;
use crate::codegen::{CodeGen, CodeGenerator};
use inkwell::values::BasicMetadataValueEnum;

impl CodeGen for CallNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let function = code_generator.module.get_function(&*self.identifier_node.identifier_token.name).unwrap();

        let parameters: Vec<BasicMetadataValueEnum> = self.parameters.iter_mut().map(|parameter| {
            parameter.build(code_generator);
            code_generator.last_assign.take().unwrap().into()
        }).collect();

        let call = code_generator.builder.build_call(function, &*parameters, "call").unwrap();

        let expression = call.try_as_basic_value().basic().unwrap();

        code_generator.last_assign = Some(expression.into_int_value())
    }
}