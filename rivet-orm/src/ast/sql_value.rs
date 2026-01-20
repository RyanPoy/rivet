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
    /// 返回二元操作符等于（=）的字符串表示。
    /// Returns the string representation of the binary operator equal (=).
    fn binary_op_eq(&self) -> &'static str {
        "="
    }
    fn binary_op_neq(&self) -> &'static str {
        "<>"
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
                fn binary_op_eq(&self) -> &'static str {
                    match self {
                        Some(_) => "=",
                        None => "IS",
                    } 
                }
                fn binary_op_neq(&self) -> &'static str {
                       match self {
                        Some(_) => "<>",
                        None => "IS NOT",
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