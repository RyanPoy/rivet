use crate::ast::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Eq,
    Ne,
    Is,
    IsNot,
    Gt,
    Gte,
    Lt,
    Lte,
}

/// 表达式枚举，用于表示SQL语句中的表达式。
/// An enum for representing expressions in SQL statements.
///
/// # 变体
/// * `Binary` - 二元运算符表达式，由操作符和两个操作数组成。
/// * `Binary` - A binary operator expression, consisting of an operator and two operands.
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    /// 代表二元运算的表达式。
    /// Represents a binary operation expression.
    ///
    /// # 字段
    /// * `left` - 左操作数，必须是一个静态字符串引用。
    /// * `left` - The left operand, must be a static string reference.
    ///
    /// * `op` - 操作符，例如 "+" 或 "-"，必须是一个静态字符串引用。
    /// * `op` - The operator, such as "+" or "-", must be a static string reference.
    ///
    /// * `right` - 右操作数，实现了 `ToSql` trait 的任意类型。
    /// * `right` - The right operand, any type that implements the `ToSql` trait.
    Binary {
        left: &'static str,
        op: Op,
        right: Value,
    },
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "expr_test.rs"]
mod tests;
