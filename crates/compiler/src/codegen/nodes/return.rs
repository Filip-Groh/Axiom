use crate::ast::ReturnNode;
use crate::codegen::{CodeGen, CodeGenerator};

impl CodeGen for ReturnNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);
        
        let expression = code_generator.last_assign.take().unwrap();
        
        code_generator.builder.build_return(Some(&expression)).unwrap();
    }
}