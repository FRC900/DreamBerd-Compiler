use std::io;

use dreamberd_compiler::{interpreter::{self, eval}, lexer::parser};
use chumsky::Parser;
use clap::Parser as CliParser;
use colored::Colorize;

/// Simple program to greet a person
#[derive(CliParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use the interpreter
    #[arg(short, long)]
    interpret: bool,

    input: Option<String>
}

fn main() {
    let cli = Cli::parse();
    if cli.interpret {
        let mut vars = Vec::new();
        let mut funcs = Vec::new();
        let stdin = io::stdin();
        let mut line = String::new();
        loop {
            
            line.clear();
            stdin.read_line(&mut line).unwrap();
            if let Some('\n') = line.chars().next_back() {
                line.pop();
            }
            if let Some('\r') = line.chars().next_back() {
                line.pop();
            }

            match parser().parse(line.as_str()) {
                Ok(ast) => match eval(&ast, &mut vars, &mut funcs) {
                    Ok(output) => println!(">>> {}", output),
                    Err(eval_err) => {
                        println!("Evaluation error: {}", eval_err);
                        println!("Local Variables:");
                        for local in &vars {
                            println!("  ${} = {}", local.0, local.1);
                        }
                        if vars.is_empty() {
                            println!("{}", "empty".italic());
                        }

                        println!("Funtions:");
                        for func in &funcs {
                            println!("  @{}({:?})", func.0, func.1);
                        }
                        if vars.is_empty() {
                            println!("{}", "empty".italic());
                        }

                    },
                },
                Err(parse_errs) => parse_errs
                    .into_iter()
                    .for_each(|e| println!("Parse error: {}", e)),
            }   
        }
    } else { 
        // Ensure that the extension is correct. 
        let path = cli.input.unwrap();
        if !path.ends_with(".db3") {
            println!("Error: Given file doesn't end with the correct extension (.db3)");
            return;
        }
        let src = std::fs::read_to_string(path).unwrap();
        

        match parser().parse(src) {
            Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
                Ok(output) => println!("{}", output),
                Err(eval_err) => println!("Evaluation error: {}", eval_err),
            },
            Err(parse_errs) => parse_errs
                .into_iter()
                .for_each(|e| println!("Parse error: {}", e)),
        }
    };    
}
