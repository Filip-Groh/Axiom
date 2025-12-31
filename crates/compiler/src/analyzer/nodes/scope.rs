use crate::analyzer::Analyzer;
use crate::ast::{ScopeNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for ScopeNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        symbol_table.push();

        for statement in &mut self.statements {
            statement.analyze(symbol_table, errors);
        }

        symbol_table.pop();
    }
}