mod lexer;
mod token;
mod user_input;
mod parser;
mod ast;
mod error;
mod analyzer;

// use inkwell::context::Context;
use std::error::Error;
use crate::analyzer::Analyzer;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    // let context = Context::create();
    // let module = context.create_module("test");
    // let builder = context.create_builder();
    //
    // let function = module.add_function("myFunction", context.void_type().fn_type(&[], false), None);
    // let basic_block = context.append_basic_block(function, "entry");
    //
    // builder.position_at_end(basic_block);
    //
    // builder.build_return(None)?;
    //
    // println!("{}", module.print_to_string().to_string());

    loop {
        println!("Input: ");
        let input = user_input::input_line();
        let tokens = Lexer::new(&input).parse()?;
        if tokens.len() == 0 {break}
        println!("Tokens: \n{:?}", tokens);
        let ast = Parser::new(tokens).parse()?;
        println!("AST: ");
        ast.display(0);
        let mut analyzer = Analyzer::new();
        let errors = analyzer.analyze(&ast);
        errors.iter().for_each(|err| println!("{}", err));
    }

    Ok(())
}