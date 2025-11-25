use inkwell::basic_block::BasicBlock;
use crate::analyzer::Analyzer;
use crate::ast::{Node, ScopeNode};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::utils::SymbolTable;

#[derive(Debug)]
pub struct IfElseNode {
    location: Range,
    pub condition: Box<Node>,
    pub consequent: Box<ScopeNode>,
    pub conditional_alternatives: Vec<(Box<Node>, Box<ScopeNode>)>,
    pub alternative: Option<Box<ScopeNode>>,
}

impl IfElseNode {
    pub fn new(location: Range, condition: Box<Node>, consequent: Box<ScopeNode>, conditional_alternatives: Vec<(Box<Node>, Box<ScopeNode>)>, alternative: Option<Box<ScopeNode>>) -> IfElseNode {
        IfElseNode {
            location,
            condition,
            consequent,
            conditional_alternatives,
            alternative,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- if ", " ".repeat(indent * 4));
        self.condition.display(indent + 1);
        self.consequent.display(indent + 1);

        for conditional_alternative in &self.conditional_alternatives {
            println!("{}- else if ", " ".repeat(indent * 4));
            conditional_alternative.0.display(indent + 1);
            conditional_alternative.1.display(indent + 1);
        }

        if let Some(alternative) = &self.alternative {
            println!("{}- else ", " ".repeat(indent * 4));
            alternative.display(indent + 1);
        }
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

        if let Some(node) = self.conditional_alternatives.iter().map(|(condition_node, consequent_node)| vec![condition_node.get_node_at(position), consequent_node.get_node_at(position)]).flatten().filter(|node| node.is_some()).next() {
            return node;
        }
        
        if let Some(node) = &self.alternative {
            node.get_node_at(position)
        } else {
            None
        }
    }
}

impl Location for IfElseNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

impl Analyzer for IfElseNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.condition.analyze(symbol_table, errors);
        self.consequent.analyze(symbol_table, errors);

        let condition_data_type = self.condition.data_type();

        if *condition_data_type != DataType::Bool {
            errors.push(AxiomError::WrongDataType(self.condition.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
        }

        for conditional_alternative in &mut self.conditional_alternatives {
            conditional_alternative.0.analyze(symbol_table, errors);
            conditional_alternative.1.analyze(symbol_table, errors);

            let condition_data_type = conditional_alternative.0.data_type();

            if *condition_data_type != DataType::Bool {
                errors.push(AxiomError::WrongDataType(conditional_alternative.0.location(), Box::from(DataType::Bool), Box::from(condition_data_type.clone())))
            }
        }

        if let Some(alternative) = &mut self.alternative {
            alternative.analyze(symbol_table, errors);
        }
    }
}

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