use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'src str),
    Op(&'src str),
    Ctrl(char),
    Ident(&'src str),
    Print,
    ConstConst,
    ConstVar,
    VarConst, 
    VarVar,
}