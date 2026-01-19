use std::fmt::{Debug, Formatter};

pub trait ToSql: Debug {
    fn to_sql(&self) -> String;
}
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

////////////

pub trait SqlValue<T>: ToSql {
    fn binary_op_eq(&self) -> &'static str {
        "="
    }
}
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
            }
            impl SqlValue<Option<$t>> for $t {}
        )*
    };
}
impl_SqlValue!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, String
);

impl SqlValue<String> for &str {}
impl SqlValue<&str> for String {}

impl SqlValue<Option<String>> for &str {}
impl SqlValue<Option<&str>> for String {}
