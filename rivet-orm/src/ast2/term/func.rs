use crate::ast2::term::expr::Expr;

#[derive(Debug, Clone)]
pub enum FuncArg {
    Expr(Expr),
    Wildcard,
}
