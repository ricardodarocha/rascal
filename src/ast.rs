#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Number(f64), //Todo! Change to Extended ?
    Date(i64, i64, i64, String), // (07/02/2026, 'd/m/a')
    String(String),
    Bool(bool),
    Variable(String),
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Unary {
        op: String,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    IndexAssign {
        array: String,
        index: Expr,
        value: Expr,
    },
    Expression(Expr),
}
