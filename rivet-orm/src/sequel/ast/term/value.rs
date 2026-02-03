use crate::sequel::ast::Scalar;
use crate::sequel::build::Binder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Single(Scalar),
    List(Vec<Scalar>),
}

impl<T: Into<Scalar>> From<T> for Value {
    fn from(value: T) -> Self {
        Value::Single(value.into())
    }
}

impl<T: Into<Scalar>> From<Vec<T>> for Value {
    fn from(values: Vec<T>) -> Self {
        let scalars: Vec<Scalar> = values.into_iter().map(|s| s.into()).collect();
        Value::List(scalars)
    }
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

#[cfg(test)]
#[path = "value_test.rs"]
mod tests;
