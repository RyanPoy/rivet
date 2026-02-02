use crate::sequel::build::Binder;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Single(Scalar),
    List(Vec<Scalar>),
}
impl Value {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Value::Single(s) => binder.bind(s.clone()),
            Value::List(vs) => {
                let placeholders: Vec<String> = vs.iter().map(|s| binder.bind(s.clone())).collect();
                format!("({})", placeholders.join(","))
            }
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl<T: IntoScalar> IntoValue for T {
    fn into_value(self) -> Value {
        Value::Single(self.into_scalar())
    }
}

impl<T: IntoScalar> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            Some(t) => Value::Single(t.into_scalar()),
            None => Value::Single(Scalar::Null),
        }
    }
}

impl<T: IntoScalar> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::List(self.into_iter().map(|t| t.into_scalar()).collect())
    }
}

impl<T: IntoScalar> IntoValue for Vec<Option<T>> {
    fn into_value(self) -> Value {
        Value::List(
            self.into_iter()
                .map(|t| match t {
                    Some(s) => s.into_scalar(),
                    None => Scalar::Null,
                })
                .collect(),
        )
    }
}

#[cfg(test)]
#[path = "value_test.rs"]
mod tests;
