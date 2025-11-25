use crate::analyzer::Analyzer;
use crate::ast::{Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::utils::SymbolTable;

#[derive(Debug)]
pub struct TernaryNode {
    location: Range,
    pub data_type: DataType,
    pub condition: Box<Node>,
    pub consequent: Box<Node>,
    pub alternative: Box<Node>,
}

impl TernaryNode {
    pub fn new(location: Range, condition: Box<Node>, consequent: Box<Node>, alternative: Box<Node>) -> TernaryNode {
        TernaryNode {
            location,
            data_type: DataType::ToBeInferred,
            condition,
            consequent,
            alternative,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- ", " ".repeat(indent * 4));
        self.condition.display(indent + 1);
        println!("{}- ? ", " ".repeat(indent * 4));
        self.consequent.display(indent + 1);
        println!("{}- : ", " ".repeat(indent * 4));
        self.alternative.display(indent + 1);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        if position.is_in_range(&self.condition.location()) {
            return self.condition.get_node_at(position);
        }

        if position.is_in_range(&self.consequent.location()) {
            return self.consequent.get_node_at(position);
        }

        self.alternative.get_node_at(position)
    }
}

impl Location for TernaryNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

impl Analyzer for TernaryNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.condition.analyze(symbol_table, errors);
        self.consequent.analyze(symbol_table, errors);
        self.alternative.analyze(symbol_table, errors);

        let condition_data_type = self.condition.data_type();
        let consequent_data_type = self.consequent.data_type();
        let alternative_data_type = self.alternative.data_type();

        if *condition_data_type != DataType::Bool {
            errors.push(AxiomError::WrongDataType(self.condition.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
        }

        if *alternative_data_type != *consequent_data_type {
            errors.push(AxiomError::WrongDataType(self.alternative.location(), Box::from(consequent_data_type.clone()), Box::from(alternative_data_type.clone())))
        }

        self.data_type = consequent_data_type.clone();
    }
}

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