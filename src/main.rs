mod lexer;
mod token;
mod parser;
mod ast;
mod error;
mod analyzer;
mod codegen;
mod utils;
mod datatype;
mod lsp;

use std::error::Error;
use clap::{Args, Command, Parser as ArgsParser, Subcommand};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::{InitializationConfig, Target};
use crate::analyzer::Analyzer;
use crate::codegen::{CodeGen, CodeGenerator};
use crate::datatype::DataType;
use crate::lexer::Lexer;
use crate::lsp::LSP;
use crate::parser::Parser;
use crate::utils::SymbolTable;

#[derive(ArgsParser, Debug)]
struct Options {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
    Build(BuildArgs),
    Lsp(LSPArgs)
}

#[derive(Args, Debug)]
struct RunArgs {

}

#[derive(Args, Debug)]
struct BuildArgs {

}

#[derive(Args, Debug)]
struct LSPArgs {

}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    match options.command {
        Commands::Run(_) => {}
        Commands::Build(_) => {}
        Commands::Lsp(_) => {
            let mut lsp = LSP::new();
            lsp.start()
        }
    }

//     let input = r#"
// function add(a: i32, b: i32): i32 {
//     return a + b
// }
//
// function main(): bool {
//     let a = 5
//     let b = 1
//     let c = add(a, b * 5)
//     c += 10
//     let d = add(1, 1)
//     return c / (d * 2) == 5
// }
//     "#.to_string();
//
//     let tokens = Lexer::new(&input).parse()?;
//     println!("Tokens: \n{:?}", tokens);
//
//     let mut ast = Parser::new(tokens).parse()?;
//     println!("AST: ");
//     ast.display(0);
//
//     let mut symbol_table = SymbolTable::new();
//     let mut errors = vec![];
//
//     symbol_table.add("i32".to_string(), DataType::Type(Box::from(DataType::I32)));
//     symbol_table.add("bool".to_string(), DataType::Type(Box::from(DataType::Bool)));
//     // Add build-in functions here!
//
//     ast.analyze(&mut symbol_table, &mut errors);
//     errors.iter().for_each(|err| println!("{}", err));
//
//     let context = Context::create();
//     let mut codegen = CodeGenerator::new(&context);
//     ast.build(&mut codegen);
//     println!("LLVM IR Code: \n{}", codegen.to_string());
//
//     Target::initialize_native(&InitializationConfig::default())?;
//     let execution_engine = codegen.module.create_jit_execution_engine(OptimizationLevel::None)?;
//     let add_func = unsafe {
//         execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
//     }.map_err(|e| format!("Function 'main' not found in JIT engine: {:?}", e))?;
//     let result = unsafe { add_func.call() };
//     println!("Result: {}", result);

    // loop {
    //     println!("Input: ");
    //     let input = user_input::input_line();
    //
    //     let tokens = Lexer::new(&input).parse()?;
    //     if tokens.len() == 0 {break}
    //     println!("Tokens: \n{:?}", tokens);
    //
    //     let ast = Parser::new(tokens).parse()?;
    //     println!("AST: ");
    //     ast.display(0);
    //
    //     let mut analyzer = Analyzer::new();
    //     let errors = analyzer.analyze(&ast);
    //     errors.iter().for_each(|err| println!("{}", err));
    //
    //     let context = Context::create();
    //     let mut codegen = CodeGenerator::new(&context);
    //     codegen.build(&ast);
    //     println!("LLVM IR Code: \n{}", codegen.to_string())
    // }

    Ok(())
}