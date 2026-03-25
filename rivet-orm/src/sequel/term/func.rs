use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::literal::Literal;
use rivet_utils::impl_into_vec_for;
use rivet_utils::into_vec::IntoVec;

#[derive(Debug, Clone)]
pub enum FuncArg {
    Wildcard,
    Expr(Expr),
}
impl_into_vec_for!(FuncArg => [Column, Expr, SelectStatement, Literal, FuncArg]);

impl From<Column> for FuncArg {
    fn from(col: Column) -> Self {
        Self::Expr(Expr::Column(col))
    }
}

impl From<Expr> for FuncArg {
    fn from(expr: Expr) -> Self {
        FuncArg::Expr(expr)
    }
}
impl From<SelectStatement> for FuncArg {
    fn from(stmt: SelectStatement) -> Self {
        Self::Expr(Expr::Subquery(Box::new(stmt)))
    }
}

impl From<Literal> for FuncArg {
    fn from(lit: Literal) -> Self {
        Self::Expr(Expr::Literal(lit))
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub args: Vec<FuncArg>,
    pub distinct: bool,
}

impl Func {
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }
}

pub fn func(name: impl Into<String>, args: impl IntoVec<FuncArg>) -> Func {
    Func {
        name: name.into(),
        args: args.to_vec(),
        distinct: false,
    }
}
macro_rules! define_functions {
    ($($name:ident),*) => {
        $(
            #[inline]
            pub fn $name(args: impl IntoVec<FuncArg>) -> Func { func(stringify!($name).to_uppercase(), args) }
        )*
    };
}
define_functions!(
    sum, avg, sqrt, abs, upper, lower, max, min, ceil, floor, exists, count, coalesce
);

// 处理 count(*)
#[inline]
pub fn count_all() -> Func {
    func("COUNT", vec![FuncArg::Wildcard])
}
