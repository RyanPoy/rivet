use crate::sequel::ast::{IntoScalar, Scalar};
use crate::sequel::build::Binder;

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
