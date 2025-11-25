use crate::analyzer::Analyzer;
use crate::ast::{Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::token::{NumberToken};
use crate::utils::SymbolTable;

#[derive(Clone)]
#[derive(Debug)]
pub struct NumberNode {
    location: Range,
    pub data_type: DataType,
    pub number_token: NumberToken
}

impl NumberNode {
    pub fn new(location: Range, number_token: NumberToken) -> NumberNode {
        NumberNode {
            location,
            data_type: DataType::I32,
            number_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.number_token.value);
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }

        Some(Box::from(Node::Number(self.clone())))
    }
}

impl Location for NumberNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

impl Analyzer for NumberNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {

    }
}

impl CodeGen for NumberNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        let number = self.number_token.value.parse::<u64>().unwrap();
        
        let value = code_generator.context.i32_type().const_int(number, false);
        
        code_generator.last_assign = Some(value);
    }
}