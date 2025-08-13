use crate::analyzer::Analyzer;
use crate::ast::{Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::utils::SymbolTable;

pub struct ScopeNode {
    pub location: Location,
    pub statements: Vec<Box<Node>>
}

impl ScopeNode {
    pub fn new(location: Location, statements: Vec<Box<Node>>) -> ScopeNode {
        ScopeNode {
            location,
            statements
        }
    }
    
    pub fn display(&self, indent: usize) {
        println!("{}- {{", " ".repeat(indent * 4));
        for statement in &self.statements {
            statement.display(indent + 1);
        }
        println!("{}}}", " ".repeat(indent * 4));
    }
}

impl Analyzer for ScopeNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        symbol_table.push();

        for statement in &mut self.statements {
            statement.analyze(symbol_table, errors);
        }

        symbol_table.pop();
    }
}

impl CodeGen for ScopeNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        code_generator.variables.push();

        for statement in &mut self.statements {
            statement.build(code_generator);
        }

        code_generator.variables.pop();
    }
}