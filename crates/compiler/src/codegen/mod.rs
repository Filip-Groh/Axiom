mod nodes;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{FunctionType};
use inkwell::values::{FunctionValue, IntValue, PointerValue};
use crate::utils::SymbolTable;

pub trait CodeGen {
    fn build(&mut self, code_generator: &mut CodeGenerator);
}

#[derive(Clone)]
pub struct FunctionContext<'function> {
    pub function_value: FunctionValue<'function>,
    pub function_type: FunctionType<'function>,
    pub parameter_names: Vec<String>,
}

pub struct CodeGenerator<'code_generator> {
    pub context: &'code_generator Context,
    pub module: Module<'code_generator>,
    pub builder: Builder<'code_generator>,
    pub variables: SymbolTable<String, PointerValue<'code_generator>>,
    pub last_assign: Option<IntValue<'code_generator>>,
    pub current_function_context: Option<FunctionContext<'code_generator>>
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
            current_function_context: None
        }
    }

    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
}