use crate::ast::FileNode;
use crate::codegen::{CodeGen, CodeGenerator};

impl CodeGen for FileNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        for function in &mut self.functions {
            function.build(code_generator);
        }
    }
}