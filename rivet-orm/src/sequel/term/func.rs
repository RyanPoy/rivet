use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::select_item::SelectItem;

#[derive(Debug, Clone)]
pub enum FuncArg {
    Wildcard,
    Expr(Expr),
    Subquery(Box<SelectStatement>),
}

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
        FuncArg::Subquery(Box::from(stmt))
    }
}

pub trait IntoFuncArgs {
    fn into_func_args(self) -> Vec<FuncArg>;
}

impl IntoFuncArgs for FuncArg {
    fn into_func_args(self) -> Vec<FuncArg> {
        vec![self]
    }
}

impl IntoFuncArgs for Column {
    fn into_func_args(self) -> Vec<FuncArg> {
        vec![FuncArg::Expr(self.into())]
    }
}

impl IntoFuncArgs for Expr {
    fn into_func_args(self) -> Vec<FuncArg> {
        vec![FuncArg::Expr(self.into())]
    }
}

impl IntoFuncArgs for SelectStatement {
    fn into_func_args(self) -> Vec<FuncArg> {
        vec![FuncArg::Expr(self.into())]
    }
}

impl<T> IntoFuncArgs for Vec<T>
where
    T: Into<FuncArg>,
{
    fn into_func_args(self) -> Vec<FuncArg> {
        self.into_iter().map(|x| x.into()).collect()
    }
}

impl<T, const N: usize> IntoFuncArgs for [T; N]
where
    T: Into<FuncArg>,
{
    fn into_func_args(self) -> Vec<FuncArg> {
        self.into_iter().map(Into::into).collect()
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

pub fn func(name: impl Into<String>, args: impl IntoFuncArgs, distinct: bool) -> Func {
    Func {
        name: name.into(),
        args: args.into_func_args(),
        distinct,
    }
}

macro_rules! define_functions {
    ($($name:ident),*) => {
        $(
            #[inline]
            pub fn $name(args: impl IntoFuncArgs) -> Func {
                // 使用 stringify!($name).to_uppercase() 自动转为大写
                func(stringify!($name).to_uppercase(), args.into_func_args(), false)
            }
        )*
    };
}
define_functions!(sum, avg, sqrt, abs, upper, lower, max, min, ceil, floor, exists, count);

#[macro_export]
macro_rules! coalesce {
    ($($arg:expr),*) => {
        {
            let mut args = Vec::new();
            $(
                // 利用 Into<Expr> 自动处理 Subquery, ColumnRef, Literal 等
                let expr: $crate::sequel::term::expr::Expr = $arg.into();
                args.push($crate::sequel::term::func::FuncArg::Expr{ expr, distinct:false });
            )*
            $crate::sequel::term::func::func("COALESCE", args)
        }
    };
}

// 处理 count(*)
#[inline]
pub fn count_all() -> Func {
    func("COUNT", vec![FuncArg::Wildcard], false)
}

impl Into<SelectItem> for Func {
    fn into(self) -> SelectItem {
        Expr::Func(self).into()
    }
}
