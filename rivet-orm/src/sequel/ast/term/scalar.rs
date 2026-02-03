use core::fmt;
use std::fmt::Formatter;

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

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Scalar::Null => write!(f, "NULL"),
            Scalar::I8(v) => write!(f, "{}", v),
            Scalar::I16(v) => write!(f, "{}", v),
            Scalar::I32(v) => write!(f, "{}", v),
            Scalar::I64(v) => write!(f, "{}", v),
            Scalar::I128(v) => write!(f, "{}", v),
            Scalar::U8(v) => write!(f, "{}", v),
            Scalar::U16(v) => write!(f, "{}", v),
            Scalar::U32(v) => write!(f, "{}", v),
            Scalar::U64(v) => write!(f, "{}", v),
            Scalar::U128(v) => write!(f, "{}", v),
            Scalar::Bool(v) => write!(f, "{}", v),
            Scalar::String(v) => write!(f, "{}", v),
        }
    }
}

macro_rules! impl_from_for_scalar {
    ($($t:ty => $variant:ident), *) => {
        $(
            impl From<$t> for Scalar {
                fn from(v: $t) -> Self {
                    Scalar::$variant(v)
                }
            }
        )*
    };
}
impl_from_for_scalar!(
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

impl From<&str> for Scalar {
    fn from(v: &str) -> Self {
        Scalar::String(v.to_string())
    }
}

impl<T> From<Option<T>> for Scalar
where
    T: Into<Scalar>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(inner) => inner.into(),
            None => Scalar::Null,
        }
    }
}
