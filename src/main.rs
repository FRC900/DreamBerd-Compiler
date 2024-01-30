use chumsky::Parser;
use dreamberd_compiler::{interpreter::eval, lexer::parser};

fn main() {
    let src = std::env::args().nth(1).unwrap();

    match parser().parse(src) {
        Ok(ast) => match eval(&ast) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
