use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;

#[derive(Debug, Clone)]
pub enum FuncArg {
    Wildcard,
    Expr { expr: Expr, distinct: bool },
    Subquery(Box<SelectStatement>),
}

impl From<ColumnRef> for FuncArg {
    fn from(col: ColumnRef) -> Self {
        Self::Expr {
            expr: Expr::Column(col),
            distinct: false,
        }
    }
}

impl From<Expr> for FuncArg {
    fn from(expr: Expr) -> Self {
        FuncArg::Expr { expr, distinct: false }
    }
}

impl From<SelectStatement> for FuncArg {
    fn from(stmt: SelectStatement) -> Self {
        FuncArg::Subquery(Box::from(stmt))
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub args: Vec<FuncArg>,
}
pub fn func(name: impl Into<String>, args: Vec<impl Into<FuncArg>>) -> Expr {
    let func = Func {
        name: name.into(),
        args: args.into_iter().map(|a| a.into()).collect(),
    };
    Expr::Func(func)
}

macro_rules! define_math_functions {
    ($($name:ident),*) => {
        $(
            #[inline]
            pub fn $name(arg: impl Into<Expr>) -> Expr {
                // 使用 stringify!($name).to_uppercase() 自动转为大写
                func(stringify!($name).to_uppercase(), vec![FuncArg::Expr{expr: arg.into(), distinct: false}])
            }
        )*
    };
}
define_math_functions!(sum, avg, sqrt, abs, upper, lower, max, min, ceil, floor);
#[inline]
pub fn exists(arg: impl Into<SelectStatement>) -> Expr {
    let arg = FuncArg::from(arg.into());
    func("EXISTS", vec![arg])
}

#[macro_export]
macro_rules! coalesce {
    ($($arg:expr),*) => {
        {
            let mut args = Vec::new();
            $(
                // 利用 Into<Expr> 自动处理 Subquery, ColumnRef, Literal 等
                let expr: $crate::ast2::term::expr::Expr = $arg.into();
                args.push($crate::ast2::term::func::FuncArg::Expr{ expr, distinct:false });
            )*
            $crate::ast2::term::func::func("COALESCE", args)
        }
    };
}

#[inline]
pub fn count(arg: impl Into<FuncArg>) -> Expr {
    func("COUNT", vec![arg])
}

// 处理 count(*)
#[inline]
pub fn count_all() -> Expr {
    func("COUNT", vec![FuncArg::Wildcard])
}
