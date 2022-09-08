use std::io::{self, Write, BufRead};
use std::fs;
use std::env;

use mamba::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Mamba REPL");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();

            let mut lexer = Lexer::new(line);
            let tokens = lexer.get_tokens().unwrap();

            for token in tokens {
                println!("{:?}", token);
            }
        }
    } else {
        let file = &args[1];
        let source: String = fs::read_to_string(file).unwrap();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.get_tokens().unwrap();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}
