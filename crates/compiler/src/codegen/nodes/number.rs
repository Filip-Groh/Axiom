use crate::ast::NumberNode;
use crate::codegen::{CodeGen, CodeGenerator};

impl CodeGen for NumberNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let number = self.number_token.value.parse::<u64>().unwrap();
        
        let value = code_generator.context.i32_type().const_int(number, false);
        
        code_generator.last_assign = Some(value);
    }
}