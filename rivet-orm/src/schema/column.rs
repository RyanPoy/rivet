use crate::ast::expr::{Expr, Op};
use crate::ast::value::ToValue;
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
pub struct Column<T: private::ColumnType> {
    pub name: &'static str,
    _marker: PhantomData<T>,
}

impl<T: private::ColumnType> Column<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }

    pub fn eq<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Eq, v.to_value())
    }

    pub fn ne<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Ne, v.to_value())
    }

    pub fn gt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Gt, v.to_value())
    }
    pub fn gte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Gte, v.to_value())
    }

    pub fn lt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Lt, v.to_value())
    }

    pub fn lte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Lte, v.to_value())
    }
}

#[allow(private_bounds)]
impl Column<String> {
    pub fn like<V: ToValue<String>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::Like, v.to_value())
    }
    pub fn not_like<V: ToValue<String>>(&self, v: V) -> Expr {
        Expr::new_binary(self.name, Op::NotLike, v.to_value())
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "column_test.rs"]
mod tests;
