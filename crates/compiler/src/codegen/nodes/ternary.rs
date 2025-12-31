use crate::ast::TernaryNode;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;

impl CodeGen for TernaryNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.condition.build(code_generator);
        let condition = code_generator.last_assign.take().unwrap();

        let then_block = code_generator.context.append_basic_block(code_generator.current_function_context.clone().unwrap().function_value, "if.then");
        let else_block = code_generator.context.append_basic_block(code_generator.current_function_context.clone().unwrap().function_value, "if.else");
        let merge_block = code_generator.context.append_basic_block(code_generator.current_function_context.clone().unwrap().function_value, "if.merge");

        code_generator.builder.build_conditional_branch(condition, then_block, else_block).unwrap();

        code_generator.builder.position_at_end(then_block);
        self.consequent.build(code_generator);
        code_generator.builder.build_unconditional_branch(merge_block).unwrap();
        let consequent = code_generator.last_assign.take().unwrap();

        code_generator.builder.position_at_end(else_block);
        self.alternative.build(code_generator);
        code_generator.builder.build_unconditional_branch(merge_block).unwrap();
        let alternative = code_generator.last_assign.take().unwrap();

        code_generator.builder.position_at_end(merge_block);

        if self.data_type == DataType::None {
            return;
        }

        let llvm_type = match self.data_type {
            DataType::I32 => code_generator.context.i32_type(),
            DataType::Bool => code_generator.context.bool_type(),
            _ => unreachable!(),
        };

        let phi_node = code_generator.builder.build_phi(llvm_type, "result").unwrap();
        phi_node.add_incoming(&[(&consequent, then_block), (&alternative, else_block)]);

        code_generator.last_assign = Some(phi_node.as_basic_value().into_int_value());
    }
}