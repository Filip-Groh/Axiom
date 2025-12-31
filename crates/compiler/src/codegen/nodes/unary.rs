use crate::ast::{Node, UnaryNode, UnaryType};
use crate::codegen::{CodeGen, CodeGenerator};

impl CodeGen for UnaryNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        match self.operation_type {
            UnaryType::PreIncrement => {
                match &mut *self.expression {
                    Node::Identifier(identifier_node) => {
                        identifier_node.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        let pointer = code_generator.variables.get(&identifier_node.identifier_token.name).unwrap();

                        let expression = code_generator.builder.build_int_add(expression, code_generator.context.i32_type().const_int(1, false), "inc").unwrap();
                        code_generator.last_assign = Some(expression);

                        code_generator.builder.build_store(*pointer, expression).unwrap();
                    }
                    _ => {
                        self.expression.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        let expression = code_generator.builder.build_int_add(expression, code_generator.context.i32_type().const_int(1, false), "inc").unwrap();
                        code_generator.last_assign = Some(expression);
                    }
                }
            }
            UnaryType::PreDecrement => {
                match &mut *self.expression {
                    Node::Identifier(identifier_node) => {
                        identifier_node.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        let pointer = code_generator.variables.get(&identifier_node.identifier_token.name).unwrap();

                        let expression = code_generator.builder.build_int_sub(expression, code_generator.context.i32_type().const_int(1, false), "dec").unwrap();
                        code_generator.last_assign = Some(expression);

                        code_generator.builder.build_store(*pointer, expression).unwrap();
                    }
                    _ => {
                        self.expression.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        let expression = code_generator.builder.build_int_sub(expression, code_generator.context.i32_type().const_int(1, false), "dec").unwrap();
                        code_generator.last_assign = Some(expression);
                    }
                }
            }
            UnaryType::PostIncrement => {
                match &mut *self.expression {
                    Node::Identifier(identifier_node) => {
                        identifier_node.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        code_generator.last_assign = Some(expression);

                        let pointer = code_generator.variables.get(&identifier_node.identifier_token.name).unwrap();
                        let expression = code_generator.builder.build_int_add(expression, code_generator.context.i32_type().const_int(1, false), "inc").unwrap();

                        code_generator.builder.build_store(*pointer, expression).unwrap();
                    }
                    _ => {
                        self.expression.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        code_generator.last_assign = Some(expression);
                    }
                }
            }
            UnaryType::PostDecrement => {
                match &mut *self.expression {
                    Node::Identifier(identifier_node) => {
                        identifier_node.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        code_generator.last_assign = Some(expression);

                        let pointer = code_generator.variables.get(&identifier_node.identifier_token.name).unwrap();
                        let expression = code_generator.builder.build_int_sub(expression, code_generator.context.i32_type().const_int(1, false), "dec").unwrap();

                        code_generator.builder.build_store(*pointer, expression).unwrap();
                    }
                    _ => {
                        self.expression.build(code_generator);
                        let expression = code_generator.last_assign.take().unwrap();

                        code_generator.last_assign = Some(expression);
                    }
                }
            }
            UnaryType::Minus => {
                self.expression.build(code_generator);
                let expression = code_generator.last_assign.take().unwrap();

                let expression = code_generator.builder.build_int_sub(code_generator.context.i32_type().const_zero(), expression, "neg").unwrap();
                code_generator.last_assign = Some(expression);
            }
            UnaryType::Absolute => {
                self.expression.build(code_generator);
                let expression = code_generator.last_assign.take().unwrap();

                let abs_func = inkwell::intrinsics::Intrinsic::find("llvm.abs").unwrap().get_declaration(&code_generator.module, &[expression.get_type().into()]).unwrap();

                let args = &[expression.into(), code_generator.context.bool_type().const_int(0, false).into()];
                let abs_value = code_generator.builder.build_call(abs_func, args, "abs_value").unwrap().try_as_basic_value().basic().unwrap().into_int_value();
                code_generator.last_assign = Some(abs_value);
            }
            UnaryType::Not => {
                self.expression.build(code_generator);
                let expression = code_generator.last_assign.take().unwrap();

                let expression = code_generator.builder.build_xor(expression, code_generator.context.bool_type().const_int(1, false), "not").unwrap();
                code_generator.last_assign = Some(expression);
            }
        }
    }
}