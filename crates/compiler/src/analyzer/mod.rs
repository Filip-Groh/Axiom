mod nodes;

use crate::utils::symbol_table::SymbolTable;
use crate::datatype::DataType;
use crate::error::AxiomError;

pub trait Analyzer {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>);
}