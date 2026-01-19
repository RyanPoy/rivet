use std::fmt::Debug;

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
        )*
    };
}
impl_ToSql!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, String
);

/////

pub trait SqlValue<T>: ToSql {}
#[allow(non_snake_case)]
macro_rules! impl_SqlValue {
    ($($t:ty),*) => {
        $(
            impl SqlValue<$t> for $t {}
        )*
    };
}
impl_SqlValue!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, String
);
impl SqlValue<String> for &str {}
