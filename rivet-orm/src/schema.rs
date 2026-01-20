use crate::ast::expression::Expr;
use crate::ast::sql_value::SqlValue;
use std::marker::PhantomData;

/// 表示SQL表中的列。
/// Represents a column in an SQL table.
#[derive(Debug)]
pub struct Column<T> {
    /// 列的名称。
    /// The name of the column.
    pub name: &'static str,

    /// 类型标记，用于编译时类型检查。
    /// A type marker for compile-time type checking.
    _marker: PhantomData<T>,
}

impl<T> Column<T> {
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
        Self {
            name,
            _marker: PhantomData,
        }
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
    pub fn eq<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: v.binary_op_eq(),
            right: Box::new(v),
        }
    }

    pub fn neq<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: v.binary_op_neq(),
            right: Box::new(v),
        }
    }

    pub fn gt<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: ">",
            right: Box::new(v),
        }
    }

    pub fn gte<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: ">=",
            right: Box::new(v),
        }
    }

    pub fn lt<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: "<",
            right: Box::new(v),
        }
    }

    pub fn lte<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: "<=",
            right: Box::new(v),
        }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "schema_test.rs"]
mod tests;
