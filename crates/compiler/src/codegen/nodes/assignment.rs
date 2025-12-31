use crate::ast::AssignmentNode;
use crate::codegen::{CodeGen, CodeGenerator};
impl CodeGen for AssignmentNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);
        
        let expression = code_generator.last_assign.take().unwrap();
        let pointer = code_generator.variables.get(&self.identifier_node.identifier_token.name).unwrap();
        
        code_generator.builder.build_store(*pointer, expression).unwrap();
    }
}