use crate::ast::{BinaryNode, BinaryType};
use crate::codegen::{CodeGen, CodeGenerator};
use inkwell::IntPredicate;

impl CodeGen for BinaryNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.left.build(code_generator);
        let left = code_generator.last_assign.take().unwrap();

        self.right.build(code_generator);
        let right = code_generator.last_assign.take().unwrap();

        match self.operation_type {
            BinaryType::Addition => {
                let expression = code_generator.builder.build_int_add(left, right, "add").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::Subtraction => {
                let expression = code_generator.builder.build_int_sub(left, right, "sub").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::Multiplication => {
                let expression = code_generator.builder.build_int_mul(left, right, "mul").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::Division => {
                let expression = code_generator.builder.build_int_signed_div(left, right, "div").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::Equal => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::EQ, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::NotEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::NE, left, right, "ne").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::GreaterThan => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::UGT, left, right, "gt").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::GreaterThanOrEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::UGE, left, right, "ge").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::LessThan => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::ULT, left, right, "lt").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::LessThanOrEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::ULE, left, right, "le").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::ShiftLeft => {
                let expression = code_generator.builder.build_left_shift(left, right, "lsh").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::ShiftRight => {
                let expression = code_generator.builder.build_right_shift(left, right, false, "rsh").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::BitwiseOr => {
                let expression = code_generator.builder.build_or(left, right, "or").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::BitwiseAnd => {
                let expression = code_generator.builder.build_and(left, right, "and").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::Or => {
                let expression = code_generator.builder.build_or(left, right, "or").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryType::And => {
                let expression = code_generator.builder.build_and(left, right, "and").unwrap();
                code_generator.last_assign = Some(expression);
            }
        }
    }
}