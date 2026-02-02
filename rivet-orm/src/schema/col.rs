use crate::sequel::ast::{Column, Expr, IntoOperand, IntoScalar, IntoValue, Op, Operand, Scalar, Value};
use std::marker::PhantomData;

mod private {
    pub trait ColumnType {}
}

macro_rules! register_column_types {
    ($($t:ty),*) => {
        $(
            impl private::ColumnType for $t {}
        )*
    };
}
register_column_types!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String);

#[derive(Debug, Eq, PartialEq)]
pub struct Col<T: private::ColumnType> {
    pub name: &'static str,
    _marker: PhantomData<T>,
}
impl<T: private::ColumnType> IntoOperand<T> for Col<T> {
    fn into_operand(self) -> Operand {
        Operand::Column(Column::new(self.name))
    }
}

impl<T: private::ColumnType> Col<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }

    fn new_binary(&self, op: Op, value: Value) -> Expr {
        Expr::new_binary(self.name, op, value)
    }
    pub fn eq<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Eq, v.into_value())
    }

    pub fn ne<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Ne, v.into_value())
    }

    pub fn gt<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Gt, v.into_value())
    }
    pub fn gte<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Gte, v.into_value())
    }

    pub fn lt<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Lt, v.into_value())
    }

    pub fn lte<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Lte, v.into_value())
    }

    pub fn in_<I: IntoIterator<Item: IntoValue>>(&self, iter: I) -> Expr {
        let mut scalars = vec![];
        for v in iter {
            match v.into_value() {
                Value::Single(s) => scalars.push(s),
                Value::List(vs) => scalars.extend(vs),
            }
        }
        Expr::new_binary(self.name, Op::In, Value::List(scalars))
    }

    pub fn not_in<I: IntoIterator<Item: IntoValue>>(&self, iter: I) -> Expr {
        let mut scalars = vec![];
        for v in iter {
            match v.into_value() {
                Value::Single(s) => scalars.push(s),
                Value::List(vs) => scalars.extend(vs),
            }
        }
        Expr::new_binary(self.name, Op::NotIn, Value::List(scalars))
    }
}

#[allow(private_bounds)]
impl Col<String> {
    pub fn like<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::Like, v.into_value())
    }
    pub fn not_like<V: IntoValue>(&self, v: V) -> Expr {
        self.new_binary(Op::NotLike, v.into_value())
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "./col_test.rs"]
mod tests;
