use std::io::{self, Write, BufRead};
use std::fs;
use std::env;

use mamba::lexer::Lexer;
use mamba::parser::Parser;
use mamba::asm::AsmGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::Command::new("clear").status().unwrap();
        println!("Mamba REPL");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();

            let mut lexer = Lexer::new(line);
            let tokens = lexer.get_tokens().unwrap();

            let mut parser = Parser::new(tokens);
            let ast = parser.parse_all();

            let mut asm_gen = AsmGenerator::new(ast);
            let result = asm_gen.generate_asm().unwrap();

            println!("{}", result);
        }
    } else {
        let file = &args[1];
        let source: String = fs::read_to_string(file).unwrap();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.get_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse_all();

        let mut asm_gen = AsmGenerator::new(ast);
        let result = asm_gen.generate_asm().unwrap();

        println!("{}", result);
    }
}