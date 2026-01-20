use crate::ast::expression::{Expr, Op};
use std::fmt::Debug;

/// 将类型转换为SQL字符串的trait。
/// A trait for converting a type to an SQL string.
pub trait ToSql: Debug {
    /// 将自身转换为SQL字符串。
    /// Converts self to an SQL string.
    fn to_sql(&self) -> String;
}

/// 为多种类型实现 `ToSql` trait 的宏。
/// A macro for implementing the `ToSql` trait for multiple types.
#[allow(non_snake_case)]
macro_rules! impl_ToSql {
    ($($t:ty),*) => {
        $(
            impl ToSql for $t {
                fn to_sql(&self) -> String { self.to_string() }
            }
            impl ToSql for Option<$t> {
                fn to_sql(&self) -> String {
                    match self {
                        Some(x) => x.to_sql(),
                        None => "NULL".to_string()
                    }
                }
            }
        )*
    };
}
impl_ToSql!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, String
);

/// //////////////////////////////////////////////////////
/// 表示SQL值的trait。
/// A trait for representing an SQL value.
pub trait SqlValue<T>: ToSql {
    fn into_binary_expr(self, col_name: &'static str, op: Op) -> Expr
    where
        Self: Sized + 'static,
    {
        Expr::Binary {
            left: col_name,
            op,
            right: Box::new(self),
        }
    }
}
/// 为多种类型实现 `SqlValue` trait 的宏。
/// A macro for implementing the `SqlValue` trait for multiple types.
#[allow(non_snake_case)]
macro_rules! impl_SqlValue {
    ($($t:ty),*) => {
        $(
            impl SqlValue<$t> for $t {}
            impl SqlValue<Option<$t>> for Option<$t> {
                fn into_binary_expr(self, col_name: &'static str, op: Op) -> Expr
                where
                    Self: Sized + 'static,
                {
                    match (self.is_some(), op) {
                        (true,  o)       => Expr::Binary { left: col_name, op: o, right: Box::new(self), }, // 有值：原样保留操作符
                        (false, Op::Eq)  => Expr::Binary { left: col_name, op: Op::Is, right: Box::new(self), }, // 无值且是 Eq：转为 IS NULL
                        (false, Op::Neq) => Expr::Binary { left: col_name, op: Op::IsNot, right: Box::new(self), }, // 无值且是 Neq：转为 IS NOT NULL
                        (false, _)       => Expr::Empty,     // 无值且是范围查询：抹除
                    }
                }
            }
            impl SqlValue<Option<$t>> for $t {}
        )*
    };
}
impl_SqlValue!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, String
);

/// 为 `&str` 实现 `SqlValue<String>` trait。
/// Implements the `SqlValue<String>` trait for `&str`.
impl SqlValue<String> for &str {}

/// 为 `String` 实现 `SqlValue<&str>` trait。
/// Implements the `SqlValue<&str>` trait for `String`.
impl SqlValue<&str> for String {}

/// 为 `&str` 实现 `SqlValue<Option<String>>` trait。
/// Implements the `SqlValue<Option<String>>` trait for `&str`.
impl SqlValue<Option<String>> for &str {}

/// 为 `String` 实现 `SqlValue<Option<&str>>` trait。
/// Implements the `SqlValue<Option<&str>>` trait for `String`.
impl SqlValue<Option<&str>> for String {}
