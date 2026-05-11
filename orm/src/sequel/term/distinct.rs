use crate::sequel::term::expr::Expr;

#[derive(Clone, Debug)]
pub enum Distinct {
    None,
    All,
    On(Vec<Expr>),
}
