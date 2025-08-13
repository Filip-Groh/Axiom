use crate::analyzer::Analyzer;
use crate::ast::{FunctionNode};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::utils::SymbolTable;

pub struct FileNode {
    pub location: Location,
    pub functions: Vec<Box<FunctionNode>>
}

impl FileNode {
    pub fn new(location: Location, functions: Vec<Box<FunctionNode>>) -> FileNode {
        FileNode {
            location,
            functions
        }
    }
    
    pub fn display(&self, indent: usize) {
        for function in &self.functions {
            function.display(indent);
        }
    }
}

impl Analyzer for FileNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        for function in &mut self.functions {
            function.analyze(symbol_table, errors);
        }
    }
}

impl CodeGen for FileNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        for function in &mut self.functions {
            function.build(code_generator);
        }
    }
}