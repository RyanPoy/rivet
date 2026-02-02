#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scalar {
    Null,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bool(bool),
    String(String),
}

pub trait IntoScalar {
    fn into_scalar(self) -> Scalar;
}

macro_rules! impl_into_scalar_for_numeric {
    ($($t:ty => $variant:ident), *) => {
        $(
            // IntoValue
            impl IntoScalar for $t {
                fn into_scalar(self) -> Scalar {
                    Scalar::$variant(self)
                }
            }
        )*
    };
}

impl_into_scalar_for_numeric!(
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    i128 => I128,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    bool => Bool,
    String => String
);
impl IntoScalar for &String {
    fn into_scalar(self) -> Scalar {
        Scalar::String(self.clone())
    }
}
impl IntoScalar for &str {
    fn into_scalar(self) -> Scalar {
        Scalar::String(self.to_string())
    }
}
