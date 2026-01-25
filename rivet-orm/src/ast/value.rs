#[derive(Debug, PartialEq, Eq)]
pub enum Value {
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

pub trait IntoValue<T> {
    fn into_value(self) -> Value;
}
macro_rules! impl_to_value_for_numeric {
    ($($t:ty => $variant:ident), *) => {
        $(
            impl IntoValue<$t> for $t {
                fn into_value(self) -> Value {
                    Value::$variant(self)
                }
            }
            impl IntoValue<$t> for Option<$t> {
                fn into_value(self) -> Value {
                    match self {
                        Some(v) => Value::$variant(v),
                        None => Value::Null,
                    }
                }
            }
        )*
    };
}
impl_to_value_for_numeric!(
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

/// `&str` only exists as a convenience input,
/// `Value` always owns `String`.
impl IntoValue<String> for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}
impl IntoValue<String> for Option<String> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => Value::Null,
        }
    }
}
impl IntoValue<String> for &String {
    fn into_value(self) -> Value {
        Value::String(self.clone())
    }
}
impl IntoValue<String> for Option<&String> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => Value::Null,
        }
    }
}
impl IntoValue<String> for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}
impl IntoValue<String> for Option<&str> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => Value::Null,
        }
    }
}


#[cfg(test)]
#[path = "value_test.rs"]
mod tests;