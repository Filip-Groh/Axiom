use crate::analyzer::Analyzer;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::token::{NumberToken};
use crate::utils::SymbolTable;

pub struct NumberNode {
    pub location: Location,
    pub data_type: DataType,
    pub number_token: NumberToken
}

impl NumberNode {
    pub fn new(location: Location, number_token: NumberToken) -> NumberNode {
        NumberNode {
            location,
            data_type: DataType::I32,
            number_token
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {}", " ".repeat(indent * 4), self.number_token.value);
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