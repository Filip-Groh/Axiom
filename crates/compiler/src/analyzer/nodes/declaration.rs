use crate::analyzer::Analyzer;
use crate::ast::{DeclarationNode};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::utils::SymbolTable;

impl Analyzer for DeclarationNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();

        symbol_table.add(self.identifier_node.identifier_token.name.clone(), expression_data_type.clone());
        
        self.identifier_node.analyze(symbol_table, errors);
    }
}