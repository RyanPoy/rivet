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
    List(Vec<Value>),
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
                    self.map(|v| v.into_value()).unwrap_or(Value::Null)
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
    bool => Bool,
    String => String
);

/// `&str` only exists as a convenience input,
/// `Value` always owns `String`.
impl IntoValue<String> for &String {
    fn into_value(self) -> Value {
        Value::String(self.clone())
    }
}
impl IntoValue<String> for Option<&String> {
    fn into_value(self) -> Value {
        self.map(|s| s.into_value()).unwrap_or(Value::Null)
    }
}
impl IntoValue<String> for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}
impl IntoValue<String> for Option<&str> {
    fn into_value(self) -> Value {
        self.map(|s| s.into_value()).unwrap_or(Value::Null)
    }
}
impl<T, I, V> IntoValue<Vec<T>> for I
where
    V: IntoValue<T>, // 约束 T 必须是合法的列类型
    I: IntoIterator<Item = V>,
{
    fn into_value(self) -> Value {
        Value::List(self.into_iter().map(|v| v.into_value()).collect())
    }
}

#[cfg(test)]
#[path = "value_test.rs"]
mod tests;
