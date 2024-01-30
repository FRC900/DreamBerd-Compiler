#[derive(Debug)]
pub enum Expr {
    // We'll tackle what integers really are later
    Int(i64),
    Float(f64),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    Call(String, Vec<Expr>),
    // We'll also deal with this later
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>, 
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
    Empty,
}

