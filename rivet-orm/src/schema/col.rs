use crate::schema::col_type::ColType;
use crate::sequel::ast::{Expr, Op, Scalar, Value};
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq)]
pub struct Col<T: ColType> {
    pub name: &'static str,
    _marker: PhantomData<T>,
}

impl<T: ColType> Col<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }

    pub fn eq<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Eq, Value::from(scalar))
    }

    pub fn ne<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Ne, Value::from(scalar))
    }

    pub fn gt<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Gt, Value::from(scalar))
    }
    pub fn gte<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Gte, Value::from(scalar))
    }

    pub fn lt<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Lt, Value::from(scalar))
    }

    pub fn lte<V>(&self, v: V) -> Expr
    where
        V: Into<Option<T>>,
    {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Lte, Value::from(scalar))
    }

    pub fn in_<V, I>(&self, iter: I) -> Expr
    where
        V: Into<Option<T>>,
        I: IntoIterator<Item = V>,
    {
        let scalars: Vec<Scalar> = iter.into_iter().map(|e| Scalar::from(e.into())).collect();
        Expr::new_binary(self.name, Op::In, Value::from(scalars))
    }

    pub fn not_in<V, I>(&self, iter: I) -> Expr
    where
        V: Into<Option<T>>,
        I: IntoIterator<Item = V>,
    {
        let scalars: Vec<Scalar> = iter.into_iter().map(|e| Scalar::from(e.into())).collect();
        Expr::new_binary(self.name, Op::NotIn, Value::from(scalars))
    }
}

#[allow(private_bounds)]
impl Col<String> {
    pub fn like<V: Into<Option<String>>>(&self, v: V) -> Expr {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::Like, Value::from(scalar))
    }

    pub fn not_like<V: Into<Option<String>>>(&self, v: V) -> Expr {
        let scalar = Scalar::from(v.into());
        Expr::new_binary(self.name, Op::NotLike, Value::from(scalar))
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "./col_test.rs"]
mod tests;
