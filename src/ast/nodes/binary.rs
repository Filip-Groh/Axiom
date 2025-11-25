use inkwell::IntPredicate;
use crate::analyzer::Analyzer;
use crate::ast::{Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::utils::SymbolTable;

#[derive(Debug)]
pub enum BinaryType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    ShiftLeft,
    ShiftRight,
    BitwiseOr,
    BitwiseAnd,
    Or,
    And,
}

#[derive(Debug)]
pub struct BinaryNode {
    location: Range,
    pub data_type: DataType,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub operation_type: BinaryType,
}

impl BinaryNode {
    pub fn new(location: Range, left: Box<Node>, right: Box<Node>, operation_type: BinaryType) -> BinaryNode {
        BinaryNode {
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

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.left.location()) {
            return self.left.get_node_at(position);
        }

        self.right.get_node_at(position)
    }
}

impl Location for BinaryNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

impl Analyzer for BinaryNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.left.analyze(symbol_table, errors);
        self.right.analyze(symbol_table, errors);

        let left_data_type = self.left.data_type();
        let right_data_type = self.right.data_type();

        if *right_data_type != *left_data_type {
            errors.push(AxiomError::WrongDataType(self.right.location(), Box::from(left_data_type.clone()), Box::from(right_data_type.clone())))
        }

        match self.operation_type {
            BinaryType::Addition | BinaryType::Subtraction | BinaryType::Multiplication | BinaryType::Division | BinaryType::ShiftLeft | BinaryType::ShiftRight | BinaryType::BitwiseAnd | BinaryType::BitwiseOr | BinaryType::Or | BinaryType::And => self.data_type = left_data_type.clone(),

            BinaryType::Equal | BinaryType::NotEqual | BinaryType::GreaterThan | BinaryType::LessThan | BinaryType::GreaterThanOrEqual | BinaryType::LessThanOrEqual => self.data_type = DataType::Bool
        }
    }
}

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