use std::io;

pub fn input_line() -> String {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Unable to read Stdin");
    input_string
}