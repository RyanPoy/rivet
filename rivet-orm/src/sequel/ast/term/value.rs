use crate::sequel::build::Binder;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Value {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Value::List(vs) => format!("({})", vs.iter().map(|v| v.build(binder)).collect::<Vec<String>>().join(",")),
            _ => binder.bind(self.clone()),
        }
    }
}

pub trait IntoValue<T> {
    fn into_value(self) -> Value;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Column { name: &'static str, alias: Option<&'static str> },
    Value(Value),
}

impl Operand {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Operand::Column { name, alias } => binder.with_alias(binder.quote(name), alias.as_deref()),
            Operand::Value(v) => v.build(binder),
        }
    }
    pub fn alias(mut self, value: &'static str) -> Self {
        if let Operand::Column { name: ref mut a, .. } = self {
            *a = value;
        }
        self
    }
}
pub trait IntoOperand<T> {
    fn into_operand(self) -> Operand;
}

macro_rules! impl_into_value_and_into_operand_for_numeric {
    ($($t:ty => $variant:ident), *) => {
        $(
            // IntoValue
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

            // IntoOperand
            impl IntoOperand<$t> for $t {
                fn into_operand(self) -> Operand {
                    Operand::Value(self.into_value())
                }
            }
            impl IntoOperand<$t> for Option<$t> {
                fn into_operand(self) -> Operand {
                    let value = self.map(|v| v.into_value()).unwrap_or(Value::Null);
                    Operand::Value(value)
                }
            }
        )*
    };
}

impl_into_value_and_into_operand_for_numeric!(
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
        let lst = self.into_iter().map(|v| v.into_value()).collect();
        Value::List(lst)
    }
}

/// `&str` only exists as a convenience input,
/// `Operand` always owns `String`.
impl IntoOperand<String> for &String {
    fn into_operand(self) -> Operand {
        Operand::Value(self.into_value())
    }
}
impl IntoOperand<String> for Option<&String> {
    fn into_operand(self) -> Operand {
        Operand::Value(self.map(|s| s.into_value()).unwrap_or(Value::Null))
    }
}
impl IntoOperand<String> for &str {
    fn into_operand(self) -> Operand {
        Operand::Value(self.into_value())
    }
}
impl IntoOperand<String> for Option<&str> {
    fn into_operand(self) -> Operand {
        Operand::Value(self.map(|s| s.into_value()).unwrap_or(Value::Null))
    }
}
impl<T, I, V> IntoOperand<Vec<T>> for I
where
    V: IntoValue<T>, // 约束 T 必须是合法的列类型
    I: IntoIterator<Item = V>,
{
    fn into_operand(self) -> Operand {
        let lst = self.into_iter().map(|v| v.into_value()).collect();
        Operand::Value(Value::List(lst))
    }
}

#[cfg(test)]
#[path = "value_test.rs"]
mod tests;
