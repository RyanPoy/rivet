use crate::ast::expr::{Expr, Op};
use crate::ast::value::{ToValue, Value};
use std::fmt;
use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}
pub trait ColumnType: private::Sealed {}

macro_rules! register_column_types {
    ($($t:ty),*) => {
        $(
            impl private::Sealed for $t {}
            impl ColumnType for $t {}
        )*
    };
}
register_column_types!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String);

/// 表示SQL表中的列。
/// Represents a column in an SQL table.
#[derive(Debug, Eq, PartialEq)]
pub struct Column<T: ColumnType> {
    /// 列名。
    /// The name of the column.
    pub name: &'static str,

    /// 类型标记，用于编译时类型检查。
    /// A type marker for compile-time type checking.
    _marker: PhantomData<T>,
}

impl<T: ColumnType> Column<T> {
    /// 创建一个新的 `Column` 实例。
    /// Creates a new `Column` instance.
    ///
    /// # 参数
    /// * `name` - 列的名称。
    /// * `name` - The name of the column.
    ///
    /// # 返回值
    /// * 新的 `Column` 实例。
    /// * A new `Column` instance.
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }

    /// 生成一个表示列等于给定值的表达式。
    /// Generates an expression representing the column being equal to the given value.
    ///
    /// # 参数
    /// * `v` - 要比较的值。
    /// * `v` - The value to compare with.
    ///
    /// # 返回值
    /// * 表示列等于给定值的表达式。
    /// * An expression representing the column being equal to the given value.
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
