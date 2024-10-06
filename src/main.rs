use std::io::{self, Write, BufRead};
use std::fs::{self, File};
use std::env;
use std::process::Command;

use mamba::lexer::Lexer;
use mamba::parser::Parser;
use mamba::codegen::llvm::IRGen;

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
        
        println!("===== Generated IR =====\n{}", ir);

        let mut ir_file = File::create("./out/mamba.ll").unwrap();
        
        println!("Writing IR to the file...");
        write!(ir_file, "{}", ir).unwrap();

        println!("Invoking llc...");
        Command::new("llc")
            .args(["-filetype=obj", "./out/mamba.ll", "-o", "./out/mamba.o"])
            .spawn()
            .unwrap();

        println!("Invoking ld...");
        Command::new("ld")
            .args(["./out/mamba.o", "-e", "_start", "-lc", "-o", "./out/mamba"])
            .spawn()
            .unwrap();

        println!("Invoking launcher...\n");
        let output = Command::new("./out/mamba")
            .output()
            .unwrap();

        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}