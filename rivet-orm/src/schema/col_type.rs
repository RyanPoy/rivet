use crate::sequel::ast::Scalar;

pub trait ColType: Into<Scalar> + Clone {}

macro_rules! impl_col_types {
    ($($t:ty),*) => {
        $(
            impl ColType for $t {}
        )*
    };
}
impl_col_types!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String, &str);
