use crate::ast::expr::{Expr, Op};
use crate::ast::value::ToValue;
use std::marker::PhantomData;
// pub trait Scalar {}
// macro_rules! register_scalar {
//     ($($t:ty), *) => {
//         $(
//             impl Scalar for $t{}
//         )*
//     };
// }
// register_scalar!(
//     u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
// );

/// 表示SQL表中的列。
/// Represents a column in an SQL table.
#[derive(Debug)]
pub struct Column<T> {
    /// 列名。
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
    pub fn eq<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Eq,
            right: v.to_value(),
        }
    }

    pub fn ne<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Ne,
            right: v.to_value(),
        }
    }

    pub fn gt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Gt,
            right: v.to_value(),
        }
    }
    pub fn gte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Gte,
            right: v.to_value(),
        }
    }

    pub fn lt<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Lt,
            right: v.to_value(),
        }
    }

    pub fn lte<V: ToValue<T>>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: Op::Lte,
            right: v.to_value(),
        }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "column_test.rs"]
mod tests;
