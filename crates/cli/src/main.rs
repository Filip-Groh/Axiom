mod project;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::{Args, Parser as ArgsParser, Subcommand, ValueEnum};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::{InitializationConfig, Target};
use serde::{Deserialize, Serialize};
use compiler::analyzer::Analyzer;
use compiler::codegen::{CodeGen, CodeGenerator};
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::utils::SymbolTable;
use crate::project::init;

#[derive(ArgsParser, Debug)]
struct Options {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
    Build(BuildArgs),
    Lsp(LSPArgs),
    Init(InitArgs)
}

#[derive(Args, Debug)]
struct RunArgs {
    path: PathBuf
}

#[derive(Args, Debug)]
struct BuildArgs {

}

#[derive(Args, Debug)]
struct LSPArgs {

}

#[derive(Args, Debug)]
struct InitArgs {
    name: String,
    project_type: ProjectType
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
enum ProjectType {
    Bin,
    Lib
}

fn main() {
    let options = Options::parse();

    match options.command {
        Commands::Run(run_args) => {
            let errors = run(run_args.path).err();
            if let Some(errors) = errors {
                errors.iter().for_each(|error| eprintln!("{}", error));
            }
        }
        Commands::Build(_) => {}
        Commands::Lsp(_) => {
            if let Err(error) = lsp::start() {
                eprintln!("{}", error);
            }
        }
        Commands::Init(init_args) => {
            if let Err(error) = init(init_args.name, init_args.project_type) {
                eprintln!("{}", error.to_string());
            }
        }
    }
}

fn run(path_buf: PathBuf) -> Result<(), Vec<Box<dyn Error>>> {
    let file_content = match fs::read_to_string(path_buf) {
        Ok(file_content) => file_content,
        Err(error) => return Err(vec![Box::from(error)])
    };

    let tokens = match Lexer::new(&file_content).parse() {
        Ok(tokens) => tokens,
        Err(error) => return Err(vec![Box::from(error)])
    };

    println!("Tokens: ");
    tokens.iter().for_each(|token| {println!("{:?}", token)});

    let mut ast = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(error) => return Err(vec![Box::from(error)])
    };

    println!("AST: ");
    ast.display(0);

    let mut symbol_table = SymbolTable::new();
    symbol_table.add_build_in_types();

    let mut errors = vec![];

    ast.analyze(&mut symbol_table, &mut errors);
    if errors.len() > 0 {
        return Err(errors.into_iter().map(|error| Box::from(error)).collect());
    }

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context);
    ast.build(&mut codegen);

    println!("LLVM IR: ");
    println!("{}", codegen.to_string());

    if let Err(error) = Target::initialize_native(&InitializationConfig::default()) {
        return Err(vec![Box::from(error)])
    }
    let execution_engine = match codegen.module.create_jit_execution_engine(OptimizationLevel::None) {
        Ok(execution_engine) => execution_engine,
        Err(error) => return Err(vec![Box::from(error)])
    };
    let main_func = match unsafe {
        execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
    }.map_err(|e| format!("Function 'main' not found in JIT engine: {:?}", e)) {
        Ok(main_func) => main_func,
        Err(error) => return Err(vec![Box::from(error)])
    };
    let result = unsafe { main_func.call() };

    println!("Result: {}", result);

    Ok(())
}