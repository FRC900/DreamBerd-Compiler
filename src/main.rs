use dreamberd_compiler::{interpreter::{self, eval}, lexer::parser};
use chumsky::Parser;
use clap::Parser as CliParser;


/// Simple program to greet a person
#[derive(CliParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use the interpreter
    #[arg(short, long)]
    interpret: bool,

    input: String
}

fn main() {
    let cli = Cli::parse();
    let src = if cli.interpret {
        cli.input
    } else { std::fs::read_to_string(cli.input).unwrap() };


    match parser().parse(src) {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
