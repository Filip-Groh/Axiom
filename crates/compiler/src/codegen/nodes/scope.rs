use crate::ast::ScopeNode;
use crate::codegen::{CodeGen, CodeGenerator};

impl CodeGen for ScopeNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        code_generator.variables.push();

        for statement in &mut self.statements {
            statement.build(code_generator);
        }

        code_generator.variables.pop();
    }
}