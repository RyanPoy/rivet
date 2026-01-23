#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Null,
    I8(Option<i8>),
    I16(Option<i16>),
    I32(Option<i32>),
    I64(Option<i64>),
    I128(Option<i128>),
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    U128(Option<u128>),
    String(Option<String>),
    Bool(Option<bool>),
}

pub trait ToValue<T> {
    fn to_value(&self) -> Value;
}
macro_rules! impl_to_value {
    ($($t:ty => $variant:ident), *) => {
        $(
            impl ToValue<$t> for $t {
                fn to_value(&self) -> Value {
                    Value::$variant(Some(*self))
                }
            }

            impl ToValue<Option<$t>> for $t {
                fn to_value(&self) -> Value {
                    Value::$variant(Some(*self))
                }
            }

            impl ToValue<Option<$t>> for Option<$t> {
                fn to_value(&self) -> Value {
                    match self {
                        Some(v) => Value::$variant(Some(*v)),
                        None => Value::Null,
                    }
                }
            }
        )*
    };
}
impl_to_value!(
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
    bool => Bool
);

impl ToValue<String> for String {
    fn to_value(&self) -> Value {
        Value::String(Some(self.into()))
    }
}
impl ToValue<Option<String>> for String {
    fn to_value(&self) -> Value {
        Value::String(Some(self.into()))
    }
}

impl ToValue<Option<String>> for Option<String> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::String(Some(v.into())),
            None => Value::Null,
        }
    }
}

//////////////////

impl ToValue<String> for &str {
    fn to_value(&self) -> Value {
        Value::String(Some(self.to_string()))
    }
}
impl ToValue<Option<String>> for &str {
    fn to_value(&self) -> Value {
        Value::String(Some(self.to_string()))
    }
}

impl ToValue<Option<String>> for Option<&str> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::String(Some(v.to_string())),
            None => Value::Null,
        }
    }
}
