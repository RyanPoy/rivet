use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::Func;
use crate::sequel::term::literal::Literal;
use rivet_utils::impl_into_vec_for;

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub expr: Expr,
    pub alias: Option<String>,
}
impl_into_vec_for!(SelectItem => [Column, Func, Expr, Literal, SelectItem]);

impl From<Column> for SelectItem {
    fn from(value: Column) -> Self {
        Self {
            expr: Expr::Column(value),
            alias: None,
        }
    }
}

impl From<Func> for SelectItem {
    fn from(value: Func) -> Self {
        Self {
            expr: Expr::Func(value),
            alias: None,
        }
    }
}

impl From<Expr> for SelectItem {
    fn from(expr: Expr) -> Self {
        Self { expr, alias: None }
    }
}

impl From<Literal> for SelectItem {
    fn from(value: Literal) -> Self {
        Self {
            expr: Expr::Literal(value.to_lit()),
            alias: None,
        }
    }
}

// 处理 Rust 原生类型 (作为 Literal)
// 只有这些类型被明确视为 Literal，避免了把 Column 也卷进来
// e.g. "username" 这个应该是 Column("username") 而不是 Literal("username")
macro_rules! impl_from_base_type_for_select_item {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SelectItem {
                fn from(value: $t) -> Self {
                    Self { expr: Expr::Literal(Literal::from(value)), alias: None }
                }
            }
        )*
    };
}

impl_from_base_type_for_select_item!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool);
