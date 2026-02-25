use crate::ast2::term::expr::Expr;
use crate::ast2::term::select_item::SelectItem;

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Literal {
    pub fn alias(self, name: impl Into<String>) -> SelectItem {
        Expr::Literal(self).alias(name)
    }
}
