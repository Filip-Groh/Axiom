use crate::ast::DeclarationNode;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;

impl CodeGen for DeclarationNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);

        let expression = code_generator.last_assign.take().unwrap();
        let pointer = match self.expression.data_type() {
            DataType::I32 => code_generator.builder.build_alloca(code_generator.context.i32_type(), self.identifier_node.identifier_token.name.as_str()).unwrap(),
            DataType::Bool => code_generator.builder.build_alloca(code_generator.context.bool_type(), self.identifier_node.identifier_token.name.as_str()).unwrap(),
            _ => unreachable!(),
        };

        code_generator.builder.build_store(pointer, expression).unwrap();

        code_generator.variables.add(self.identifier_node.identifier_token.name.clone(), pointer);
    }
}