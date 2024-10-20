#[derive(Debug, Clone)]
pub enum Expr {
    Num(i64),
    Id(String),
    Bool(bool),
    Unit,
    UnOp(UnaryOp, Box<Expr>),
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<(String, Box<Expr>)>, Box<Expr>),
    Do(Vec<Box<Expr>>),
    FuncDef(String, Vec<String>, Box<Expr>), 
    FuncCall(Box<Expr>, Vec<Box<Expr>>),      
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Add1,
    Sub1,
    IsZero,
    IsNum,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Eq,
    Lt,
}
