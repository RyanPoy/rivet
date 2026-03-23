use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use std::ascii::AsciiExt;

#[derive(Debug, Clone)]
pub enum FuncArg {
    Wildcard,
    Expr(Expr),
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
        Self::Expr(Expr::Subquery(Box::new(stmt)))
    }
}

impl From<crate::sequel::term::literal::Literal> for FuncArg {
    fn from(lit: crate::sequel::term::literal::Literal) -> Self {
        Self::Expr(Expr::Literal(lit))
    }
}

pub trait IntoFuncArgs {
    fn into_func_args(self) -> Vec<FuncArg>;
}

macro_rules! impl_into_func_args_for_single {
    ($($t:ty),*) => {
        $(
            impl IntoFuncArgs for $t {
                fn into_func_args(self) -> Vec<FuncArg> {
                    vec![self.into()] // 直接利用已经定义好的 From<$t> for FuncArg
                }
            }
        )*
    };
}

// 定义哪些类型可以作为单数参数传入函数
impl_into_func_args_for_single!(FuncArg, Column, Expr, SelectStatement);

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

pub fn func(name: impl Into<String>, args: impl IntoFuncArgs) -> Func {
    Func {
        name: name.into(),
        args: args.into_func_args(),
        distinct: false,
    }
}

macro_rules! define_functions {
    ($($name:ident),*) => {
        $(
            #[inline]
            pub fn $name(args: impl IntoFuncArgs) -> Func {
                // 使用 stringify!($name).to_uppercase() 自动转为大写
                func(stringify!($name).to_uppercase(), args.into_func_args())
            }
        )*
    };
}
define_functions!(sum, avg, sqrt, abs, upper, lower, max, min, ceil, floor, exists, count);

// 处理 count(*)
#[inline]
pub fn count_all() -> Func {
    func("COUNT", vec![FuncArg::Wildcard])
}

/// COALESCE 函数 - 返回第一个非 NULL 的值
#[inline]
pub fn coalesce(args: impl IntoFuncArgs) -> Func {
    func("COALESCE", args.into_func_args())
}
