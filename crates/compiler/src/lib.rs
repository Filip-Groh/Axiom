pub mod lexer;
mod token;
pub mod parser;
mod ast;
pub mod error;
pub mod analyzer;
pub mod codegen;
pub mod utils;
pub mod datatype;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
