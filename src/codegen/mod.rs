use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum};
use inkwell::values::{IntValue, PointerValue};
use crate::ast::{BinaryOperationType, Node};
use crate::utils::SymbolTable;

pub struct CodeGenerator<'code_generator> {
    context: &'code_generator Context,
    module: Module<'code_generator>,
    builder: Builder<'code_generator>,
    variables: SymbolTable<String, PointerValue<'code_generator>>,
    last_assign: Option<IntValue<'code_generator>>,
}

impl<'code_generator> CodeGenerator<'code_generator> {
    pub fn new(context: &'code_generator Context) -> CodeGenerator<'code_generator> {
        let module = context.create_module("program");
        let builder = context.create_builder();
        
        CodeGenerator {
            module,
            builder,
            context,
            variables: SymbolTable::new(),
            last_assign: None,
        }
    }
    
    pub fn build(&mut self, node: &Node) {
        match node {
            Node::File(_, file_node) => {
                for function in &file_node.functions {
                    self.build(function);
                }
            }
            Node::Function(_, function_node) => {
                let function = self.module.add_function(function_node.identifier_node.identifier_token.name.as_str(), self.context.i32_type().fn_type(&*vec![BasicMetadataTypeEnum::from(self.context.i32_type()); function_node.parameters.len()], false), None);
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);

                self.variables.push();

                self.build(&function_node.scope);

                self.variables.pop();
            }
            Node::Scope(_, scope_node) => {
                self.variables.push();

                for statement in &scope_node.statements {
                    self.build(statement);
                }

                self.variables.pop();
            }
            Node::Return(_, return_node) => {
                self.build(&return_node.expression);
                let expression = self.last_assign.take().unwrap();
                self.builder.build_return(Some(&expression)).unwrap();
            }
            Node::Assignment(_, assignment_node) => {
                self.build(&assignment_node.expression);
                let expression = self.last_assign.take().unwrap();
                let i32_type = self.context.i32_type();
                let pointer = self.builder.build_alloca(i32_type, assignment_node.identifier_node.identifier_token.name.as_str()).unwrap();
                self.builder.build_store(pointer, expression).unwrap();
                self.variables.add(assignment_node.identifier_node.identifier_token.name.clone(), pointer);
            }
            Node::BinaryOperation(_, binary_operation_node) => {
                self.build(&binary_operation_node.left);
                let left = self.last_assign.take().unwrap();

                self.build(&binary_operation_node.right);
                let right = self.last_assign.take().unwrap();

                match binary_operation_node.operation_type {
                    BinaryOperationType::Addition() => {
                        let expression = self.builder.build_int_add(left, right, "add").unwrap();
                        self.last_assign = Some(expression);
                    }
                    BinaryOperationType::Subtraction() => {
                        let expression = self.builder.build_int_sub(left, right, "sub").unwrap();
                        self.last_assign = Some(expression);
                    }
                    BinaryOperationType::Multiplication() => {
                        let expression = self.builder.build_int_mul(left, right, "mul").unwrap();
                        self.last_assign = Some(expression);
                    }
                    BinaryOperationType::Division() => {
                        let expression = self.builder.build_int_signed_div(left, right, "div").unwrap();
                        self.last_assign = Some(expression);
                    }
                }
            }
            Node::Number(_, number_node) => {
                let number = number_node.number_token.value.parse::<u64>().unwrap();
                let value = self.context.i32_type().const_int(number, false);
                self.last_assign = Some(value);
            }
            Node::Identifier(_, identifier_node) => {
                let identifier = self.variables.get(&identifier_node.identifier_token.name).unwrap();
                let pointer = self.builder.build_load(*identifier, "load").unwrap();
                self.last_assign = Some(IntValue::try_from(pointer).unwrap());
            }
        }
    }

    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
}