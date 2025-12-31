use crate::ast::IdentifierNode;
use crate::codegen::{CodeGen, CodeGenerator};
use inkwell::values::IntValue;

impl CodeGen for IdentifierNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let identifier = code_generator.variables.get(&self.identifier_token.name).unwrap();

        let pointer = code_generator.builder.build_load(*identifier, "load").unwrap();

        code_generator.last_assign = Some(IntValue::try_from(pointer).unwrap());
    }
}