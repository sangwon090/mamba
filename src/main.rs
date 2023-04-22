use std::io::{self, Write, BufRead};
use std::fs;
use std::env;
use std::process::{Command, Stdio};

use mamba::lexer::Lexer;
use mamba::parser::Parser;
use mamba::irgen::IRGen;

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

            let mut irgen = IRGen::new(ast);
            let ir = irgen.generate_ir().unwrap();
        }
    } else {
        let file = &args[1];
        let source: String = fs::read_to_string(file).unwrap();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.get_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse_all();

        for statement in &ast.statements {
            println!("{}", statement.to_string());
        }

        let mut irgen = IRGen::new(ast);
        let ir = irgen.generate_ir().unwrap();
        println!("===== Generated IR =====\n{}\n", ir);

        let mut llc = Command::new("llc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();
        
        let llc_stdin = llc.stdin.as_mut().unwrap();
        llc_stdin.write_all(ir.as_bytes()).unwrap();
        drop(llc_stdin);

        let asm = llc.wait_with_output().unwrap().stdout;

        println!("===== Generated ASM =====\n{}\n", String::from_utf8_lossy(&asm));
    }
}