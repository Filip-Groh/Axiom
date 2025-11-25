use crate::analyzer::Analyzer;
pub use crate::ast::nodes::*;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::error::AxiomError;
use crate::error::location::{Location, Position, Range};
use crate::utils::SymbolTable;

mod nodes;

#[derive(Debug)]
pub enum Node {
    File(FileNode),
    Function(FunctionNode),
    Parameter(ParameterNode),
    Scope(ScopeNode),
    IfElse(IfElseNode),
    Return(ReturnNode),
    Declaration(DeclarationNode),
    Assignment(AssignmentNode),
    Ternary(TernaryNode),
    Binary(BinaryNode),
    Unary(UnaryNode),
    Number(NumberNode),
    Identifier(IdentifierNode),
    Call(CallNode),
}

impl Node {
    pub fn display(&self, indent: usize) {
        match &self {
            Node::File(file_node) => file_node.display(indent),
            Node::Number(number_node) => number_node.display(indent),
            Node::Identifier(identifier_node) => identifier_node.display(indent),
            Node::Binary(binary_operation_node) => binary_operation_node.display(indent),
            Node::Assignment(assignment_node) => assignment_node.display(indent),
            Node::Declaration(declaration_node) => declaration_node.display(indent),
            Node::Scope(scope_node) => scope_node.display(indent),
            Node::Function(function_node) => function_node.display(indent),
            Node::Return(return_node) => return_node.display(indent),
            Node::Call(call_node) => call_node.display(indent),
            Node::Parameter(parameter_node) => parameter_node.display(indent),
            Node::Ternary(ternary_node) => ternary_node.display(indent),
            Node::IfElse(if_else_node) => if_else_node.display(indent),
            Node::Unary(unary_node) => unary_node.display(indent),
        }
    }

    pub fn location(&self) -> Range {
        match &self {
            Node::File(file_node) => file_node.location(),
            Node::Number(number_node) => number_node.location(),
            Node::Identifier(identifier_node) => identifier_node.location(),
            Node::Binary(binary_operation_node) => binary_operation_node.location(),
            Node::Assignment(assignment_node) => assignment_node.location(),
            Node::Declaration(declaration_node) => declaration_node.location(),
            Node::Scope(scope_node) => scope_node.location(),
            Node::Function(function_node) => function_node.location(),
            Node::Return(return_node) => return_node.location(),
            Node::Call(call_node) => call_node.location(),
            Node::Parameter(parameter_node) => parameter_node.location(),
            Node::Ternary(ternary_node) => ternary_node.location(),
            Node::IfElse(if_else_node) => if_else_node.location(),
            Node::Unary(unary_node) => unary_node.location(),
        }
    }

    pub fn data_type(&self) -> &DataType {
        match &self {
            Node::File(_) => &DataType::None,
            Node::Number(number_node) => &number_node.data_type,
            Node::Identifier(identifier_node) => &identifier_node.data_type,
            Node::Binary(binary_operation_node) => &binary_operation_node.data_type,
            Node::Assignment(_) => &DataType::None,
            Node::Declaration(_) => &DataType::None,
            Node::Scope(_) => &DataType::None,
            Node::Function(function_node) => &function_node.data_type,
            Node::Return(_) => &DataType::None,
            Node::Call(call_node) => &call_node.data_type,
            Node::Parameter(_) => &DataType::None,
            Node::Ternary(ternary_node) => &ternary_node.data_type,
            Node::IfElse(_) => &DataType::None,
            Node::Unary(unary_node) => &unary_node.data_type,
        }
    }

    pub fn get_node_at(&self, position: &Position) -> Option<Box<Node>> {
        match &self {
            Node::File(file_node) => file_node.get_node_at(position),
            Node::Function(function_node) => function_node.get_node_at(position),
            Node::Parameter(parameter_node) => parameter_node.get_node_at(position),
            Node::Scope(scope_node) => scope_node.get_node_at(position),
            Node::IfElse(if_else_node) => if_else_node.get_node_at(position),
            Node::Return(return_node) => return_node.get_node_at(position),
            Node::Declaration(declaration_node) => declaration_node.get_node_at(position),
            Node::Assignment(assignment_node) => assignment_node.get_node_at(position),
            Node::Ternary(ternary_node) => ternary_node.get_node_at(position),
            Node::Binary(binary_operation_node) => binary_operation_node.get_node_at(position),
            Node::Unary(unary_node) => unary_node.get_node_at(position),
            Node::Number(number_node) => number_node.get_node_at(position),
            Node::Identifier(identifier_node) => identifier_node.get_node_at(position),
            Node::Call(call_node) => call_node.get_node_at(position)
        }
    }
}

impl Analyzer for Node {
    fn analyze(&mut self, symbol_table: &mut SymbolTable<String, DataType>, errors: &mut Vec<AxiomError>) {
        match self {
            Node::File(file_node) => file_node.analyze(symbol_table, errors),
            Node::Number(number_node) => number_node.analyze(symbol_table, errors),
            Node::Identifier(identifier_node) => identifier_node.analyze(symbol_table, errors),
            Node::Binary(binary_operation_node) => binary_operation_node.analyze(symbol_table, errors),
            Node::Assignment(assignment_node) =>assignment_node.analyze(symbol_table, errors),
            Node::Declaration(declaration_node) => declaration_node.analyze(symbol_table, errors),
            Node::Scope(scope_node) => scope_node.analyze(symbol_table, errors),
            Node::Function(function_node) => function_node.analyze(symbol_table, errors),
            Node::Return(return_node) => return_node.analyze(symbol_table, errors),
            Node::Call(call_node) => call_node.analyze(symbol_table, errors),
            Node::Parameter(parameter_node) => parameter_node.analyze(symbol_table, errors),
            Node::Ternary(ternary_node) => ternary_node.analyze(symbol_table, errors),
            Node::IfElse(if_else_node) => if_else_node.analyze(symbol_table, errors),
            Node::Unary(unary_node) => unary_node.analyze(symbol_table, errors),
        }
    }
}

impl CodeGen for Node {
    fn build(&mut self, code_generator: &mut CodeGenerator) {
        match self {
            Node::File(file_node) => file_node.build(code_generator),
            Node::Number(number_node) => number_node.build(code_generator),
            Node::Identifier(identifier_node) => identifier_node.build(code_generator),
            Node::Binary(binary_operation_node) => binary_operation_node.build(code_generator),
            Node::Assignment(assignment_node) =>assignment_node.build(code_generator),
            Node::Declaration(declaration_node) => declaration_node.build(code_generator),
            Node::Scope(scope_node) => scope_node.build(code_generator),
            Node::Function(function_node) => function_node.build(code_generator),
            Node::Return(return_node) => return_node.build(code_generator),
            Node::Call(call_node) => call_node.build(code_generator),
            Node::Parameter(parameter_node) => parameter_node.build(code_generator),
            Node::Ternary(ternary_node) => ternary_node.build(code_generator),
            Node::IfElse(if_else_node) => if_else_node.build(code_generator),
            Node::Unary(unary_node) => unary_node.build(code_generator),
        }
    }
}