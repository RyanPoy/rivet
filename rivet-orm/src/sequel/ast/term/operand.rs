use crate::sequel::ast::{IntoValue, Value};
use crate::sequel::build::Binder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
    name: &'static str,
    alias: Option<&'static str>,
}
impl Column {
    pub fn new(name: &'static str) -> Self {
        Self { name, alias: None }
    }

    pub fn alias(mut self, name: &'static str) -> Self {
        self.alias = Some(name);
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Column(Column),
    Value(Value),
}

impl Operand {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Operand::Column(Column { name, alias }) => binder.with_alias(binder.quote(name), alias.as_deref()),
            Operand::Value(v) => v.build(binder),
        }
    }
}
pub trait IntoOperand<T> {
    fn into_operand(self) -> Operand;
}

macro_rules! impl_into_operand_for_numeric {
    ($($t:ty), *) => {
        $(
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

impl_into_operand_for_numeric!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String);

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
