use chumsky::prelude::*;
use crate::{expression::Expr};

pub fn parser<'src>() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let int = text::int(10)
            .map(|s: String| Expr::Float(s.parse().unwrap()))
            .padded();
    
        let atom = int
            .or(expr.delimited_by(just('('), just(')'))).padded();
    
        let op = |c| just(c).padded();
    
        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));
    
        let product = unary.clone()
            .then(op('*').to(Expr::Mul as fn(_, _) -> _)
                .or(op('/').to(Expr::Div as fn(_, _) -> _))
                .then(unary)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
    
        let sum = product.clone()
            .then(op('+').to(Expr::Add as fn(_, _) -> _)
                .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                .then(product)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
    
        sum
    })
        .then_ignore(end())
}