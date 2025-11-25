use crate::analyzer::Analyzer;
use crate::ast::{DeclarationNode, FunctionNode, Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::utils::SymbolTable;

#[derive(Debug)]
pub struct FileNode {
    location: Range,
    pub functions: Vec<Box<FunctionNode>>
}

impl FileNode {
    pub fn new(location: Range, functions: Vec<Box<FunctionNode>>) -> FileNode {
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
    
    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        if !position.is_in_range(&self.location()) {
            return None;
        }
        
        self.functions.iter().map(|function_node| function_node.get_node_at(position)).filter(|node| node.is_some()).next()?
    }
}

impl Location for FileNode {
    fn location(&self) -> Range {
        self.location.clone()
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