use crate::{expression::Expr};
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pest/grammar.pest"]
struct DreamParser;

pub fn parse(input: &str) -> Vec<Expr> {
    let parse = DreamParser::parse(Rule::program, input);
    let mut ast = Vec::new();
    if let Ok(parse) = parse {
        for pair in parse.into_iter() {
            match pair.as_rule() {
                Rule::program => {
                    for statement in pair.into_inner() {
                        println!("{:?}", statement);
                        /* 
                        ast.push( match statement.as_rule() {
                            Rule::number => {
                                Expr::Int(statement.as_span().as_str().parse::<i64>().unwrap() )
                            },
                            Rule::binary_op => {
                                match statement.as_span().as_str() {
                                    "+" => { Expr::Add(, ()) }
                                    "-" => {  }
                                    "*" => {  }
                                    "/" => {  }
                                    _ => { Expr::Empty } // FIXME: This should be an error
                                }
                            }
                            _ => { Expr::Empty }
                        } )
                        */
                    }
                }
                _ => unreachable!()
            }
        }
    }
    else {
        println!("Error: {:?}", parse);
    }
    ast
}
