use crate::analyzer::Analyzer;
use crate::ast::{IdentifierNode, Node};
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::Location;
use crate::utils::SymbolTable;

pub struct DeclarationNode {
    pub location: Location,
    pub identifier_node: Box<IdentifierNode>,
    pub expression: Box<Node>,
}

impl DeclarationNode {
    pub fn new(location: Location, identifier_node: Box<IdentifierNode>, expression: Box<Node>) -> DeclarationNode {
        DeclarationNode {
            location,
            identifier_node,
            expression,
        }
    }

    pub fn display(&self, indent: usize) {
        println!("{}- let {} = ", " ".repeat(indent * 4), self.identifier_node.identifier_token.name);
        self.expression.display(indent * 4);
    }
}

impl Analyzer for DeclarationNode {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        self.expression.analyze(symbol_table, errors);

        let expression_data_type = self.expression.data_type();

        symbol_table.add(self.identifier_node.identifier_token.name.clone(), expression_data_type.clone());
    }
}

impl CodeGen for DeclarationNode {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        self.expression.build(code_generator);

        let expression = code_generator.last_assign.take().unwrap();
        let pointer = match self.expression.data_type() {
            DataType::I32 => code_generator.builder.build_alloca(code_generator.context.i32_type(), self.identifier_node.identifier_token.name.as_str()).unwrap(),
            DataType::Bool => code_generator.builder.build_alloca(code_generator.context.bool_type(), self.identifier_node.identifier_token.name.as_str()).unwrap(),
            _ => unreachable!(),
        };

        code_generator.builder.build_store(pointer, expression).unwrap();

        code_generator.variables.add(self.identifier_node.identifier_token.name.clone(), pointer);
    }
}