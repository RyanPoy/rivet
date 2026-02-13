use crate::ast2::term::expr::Expr;

pub enum SelectItem {
    Expr(Expr),
}
