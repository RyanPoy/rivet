use crate::ast::expr::{Expr, Op};
use crate::ast::value::{ToValue, Value};
use std::fmt;
use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
    impl Sealed for isize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
    impl Sealed for usize {}
    impl Sealed for bool {}
    impl Sealed for String {}
    impl Sealed for &str {}
}
pub trait ColumnType: private::Sealed {}
impl ColumnType for i8 {}
impl ColumnType for i16 {}
impl ColumnType for i32 {}
impl ColumnType for i64 {}
impl ColumnType for i128 {}
impl ColumnType for isize {}
impl ColumnType for u8 {}
impl ColumnType for u16 {}
impl ColumnType for u32 {}
impl ColumnType for u64 {}
impl ColumnType for u128 {}
impl ColumnType for usize {}
impl ColumnType for bool {}
impl ColumnType for String {}
impl ColumnType for &str {}

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
        let right = v.to_value();
        let op = match right {
            Value::Null => Op::Is,
            _ => Op::Eq,
        };
        Expr::Binary { left: self.name, op, right }
    }

    pub fn ne<V: ToValue<T>>(&self, v: V) -> Expr {
        let right = v.to_value();
        let op = match right {
            Value::Null => Op::IsNot,
            _ => Op::Ne,
        };
        Expr::Binary { left: self.name, op, right }
    }

    pub fn gt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::Gt, right: v.to_value() }
    }
    pub fn gte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::Gte, right: v.to_value() }
    }

    pub fn lt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::Lt, right: v.to_value() }
    }

    pub fn lte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::Lte, right: v.to_value() }
    }
}

trait StringType: ColumnType {}
impl StringType for String {}
impl StringType for &str {}

#[allow(private_bounds)]
impl<T: StringType> Column<T> {
    pub fn like<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::Like, right: v.to_value() }
    }
    pub fn not_like<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary { left: self.name, op: Op::NotLike, right: v.to_value() }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "column_test.rs"]
mod tests;
