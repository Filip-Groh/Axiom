use crate::analyzer::Analyzer;
use crate::ast::{FileNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for FileNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        for function in &mut self.functions {
            function.analyze(symbol_table, errors);
        }
    }
}