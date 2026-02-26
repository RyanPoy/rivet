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
macro_rules! impl_from_for_literal {
    ($variant:ident => [$($t:ty),*]) => {
        $(
            impl From<$t> for Literal {
                fn from(v: $t) -> Self {
                    Literal::$variant(v as _)
                }
            }
        )*
    };
}
impl_from_for_literal!(Int => [i8, i16, i32, i64]);
impl_from_for_literal!(Float => [f32, f64]);

impl From<&str> for Literal {
    fn from(v: &str) -> Self {
        Literal::String(v.into())
    }
}
impl From<String> for Literal {
    fn from(v: String) -> Self {
        Literal::String(v)
    }
}
impl From<bool> for Literal {
    fn from(v: bool) -> Self {
        Literal::Bool(v)
    }
}
