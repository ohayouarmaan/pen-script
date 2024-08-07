use std::{env, fs};
mod lexer;
mod parser;

fn print_usage() {
    println!("[USAGE]");
    println!("cargo run <FILE_PATH>");
    println!("example: cargo run examples/test.pen");
}

fn main() {
    let arguments = env::args().collect::<Vec<String>>();
    let first_argument = arguments.get(1);
    if let Some(file_name) = first_argument {
        let source_code = fs::read_to_string(file_name).expect("[Error]: Can not open specified file.");
        let mut primary_lexer = lexer::Lexer::new(&source_code);
        primary_lexer.lex();
        let mut p = parser::Parser::new(&primary_lexer);
        println!("{:?}", p.parse());
    } else {
        print_usage();
    }
}
