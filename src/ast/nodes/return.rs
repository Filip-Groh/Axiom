use crate::analyzer::Analyzer;
use crate::ast::{Node, ParameterNode};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Range};
use crate::utils::SymbolTable;

pub struct ReturnNode {
    location: Range,
    pub expression: Box<Node>,
}

impl ReturnNode {
    pub fn new(location: Range, expression: Box<Node>) -> ReturnNode {
        ReturnNode { 
            location,
            expression
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- return ", " ".repeat(indent * 4));
        self.expression.display(indent + 1);
    }
}

impl Location for ReturnNode {
    fn location(&self) -> Range {
        self.location.clone()
    }
}

impl Analyzer for ReturnNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();
        
        match symbol_table.get(&"return".to_string()) {
            Some(function_return_type) => {
                if *expression_data_type != *function_return_type {
                    errors.push(AxiomError::WrongDataType(self.expression.location(), Box::from(function_return_type.clone()), Box::from(expression_data_type.clone())))
                }
            }
            None => errors.push(AxiomError::WrongDataType(self.expression.location(), Box::from(DataType::None), Box::from(expression_data_type.clone())))
        }
    }
}

impl CodeGen for ReturnNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);
        
        let expression = code_generator.last_assign.take().unwrap();
        
        code_generator.builder.build_return(Some(&expression)).unwrap();
    }
}