use inkwell::IntPredicate;
use crate::analyzer::Analyzer;
use crate::ast::{Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::utils::SymbolTable;

#[derive(Debug)]
pub enum BinaryOperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual
}

pub struct BinaryOperationNode {
    pub location: Location,
    pub data_type: DataType,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub operation_type: BinaryOperationType,
}

impl BinaryOperationNode {
    pub fn new(location: Location, left: Box<Node>, right: Box<Node>, operation_type: BinaryOperationType) -> BinaryOperationNode {
        BinaryOperationNode {
            location,
            data_type: DataType::ToBeInferred,
            left,
            right,
            operation_type
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {:?}", " ".repeat(indent * 4), self.operation_type);
        self.left.display(indent + 1);
        self.right.display(indent + 1);
    }
}

impl Analyzer for BinaryOperationNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.left.analyze(symbol_table, errors);
        self.right.analyze(symbol_table, errors);

        let left_data_type = self.left.data_type();
        let right_data_type = self.right.data_type();

        if *right_data_type != *left_data_type {
            errors.push(AxiomError::WrongDataType(self.right.location().clone(), Box::from(left_data_type.clone()), Box::from(right_data_type.clone())))
        }

        match self.operation_type {
            BinaryOperationType::Addition | BinaryOperationType::Subtraction | BinaryOperationType::Multiplication | BinaryOperationType::Division => self.data_type = left_data_type.clone(),

            BinaryOperationType::Equal | BinaryOperationType::NotEqual | BinaryOperationType::GreaterThan | BinaryOperationType::LessThan | BinaryOperationType::GreaterThanOrEqual | BinaryOperationType::LessThanOrEqual => self.data_type = DataType::Bool
        }
    }
}

impl CodeGen for BinaryOperationNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.left.build(code_generator);
        let left = code_generator.last_assign.take().unwrap();

        self.right.build(code_generator);
        let right = code_generator.last_assign.take().unwrap();

        match self.operation_type {
            BinaryOperationType::Addition => {
                let expression = code_generator.builder.build_int_add(left, right, "add").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::Subtraction => {
                let expression = code_generator.builder.build_int_sub(left, right, "sub").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::Multiplication => {
                let expression = code_generator.builder.build_int_mul(left, right, "mul").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::Division => {
                let expression = code_generator.builder.build_int_signed_div(left, right, "div").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::Equal => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::EQ, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::NotEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::NE, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::GreaterThan => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::UGT, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::GreaterThanOrEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::UGE, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::LessThan => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::ULT, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
            BinaryOperationType::LessThanOrEqual => {
                let expression = code_generator.builder.build_int_compare(IntPredicate::ULE, left, right, "eq").unwrap();
                code_generator.last_assign = Some(expression);
            }
        }
    }
}