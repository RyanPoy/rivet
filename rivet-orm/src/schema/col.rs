use crate::sequel::ast::{Expr, IntoOperand, IntoValue, Op, Operand};
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
        Operand::Column { name: self.name, alias: None }
    }
}

impl<T: private::ColumnType> Col<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }

    pub fn eq<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Eq, v.into_value())
    }

    pub fn ne<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Ne, v.into_value())
    }

    pub fn gt<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Gt, v.into_value())
    }
    pub fn gte<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Gte, v.into_value())
    }

    pub fn lt<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Lt, v.into_value())
    }

    pub fn lte<V: IntoValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Lte, v.into_value())
    }

    pub fn in_<V: IntoValue<T>, I: IntoIterator<Item = V>>(&self, iter: I) -> Expr {
        Expr::new_binary(self.name, Op::In, iter.into_value())
    }

    pub fn not_in<V: IntoValue<T>, I: IntoIterator<Item = V>>(&self, iter: I) -> Expr {
        Expr::new_binary(self.name, Op::NotIn, iter.into_value())
    }
}

#[allow(private_bounds)]
impl Col<String> {
    pub fn like<V: IntoValue<String>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Like, v.into_value())
    }
    pub fn not_like<V: IntoValue<String>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::NotLike, v.into_value())
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "./col_test.rs"]
mod tests;
