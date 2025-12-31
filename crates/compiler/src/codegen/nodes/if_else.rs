use crate::ast::{IfElseNode, Node, ScopeNode};
use crate::codegen::{CodeGen, CodeGenerator};
use inkwell::basic_block::BasicBlock;

impl CodeGen for IfElseNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let merge_block = code_generator.context.append_basic_block(code_generator.current_function_context.clone().unwrap().function_value, "if.merge");

        IfElseNode::build_singe_if(code_generator, &mut self.condition, &mut self.consequent, merge_block);

        for conditional_alternative in &mut self.conditional_alternatives {
            IfElseNode::build_singe_if(code_generator, &mut conditional_alternative.0, &mut conditional_alternative.1, merge_block);
        }

        if let Some(alternative) = &mut self.alternative {
            alternative.build(code_generator);
        }

        code_generator.builder.build_unconditional_branch(merge_block).unwrap();

        code_generator.builder.position_at_end(merge_block);
    }
}

impl IfElseNode {
    fn build_singe_if(code_generator: &mut CodeGenerator, condition: &mut Box<Node>, consequent: &mut Box<ScopeNode>, merge_block: BasicBlock) {
        condition.build(code_generator);
        let condition = code_generator.last_assign.take().unwrap();

        let then_block = code_generator.context.prepend_basic_block(merge_block, "if.then");
        let else_block = code_generator.context.prepend_basic_block(merge_block, "if.else");

        code_generator.builder.build_conditional_branch(condition, then_block, else_block).unwrap();

        code_generator.builder.position_at_end(then_block);
        consequent.build(code_generator);
        code_generator.builder.build_unconditional_branch(merge_block).unwrap();

        code_generator.builder.position_at_end(else_block);
    }
}