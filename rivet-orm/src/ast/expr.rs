use crate::ast::value::{Operand, Value};

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
    Like,
    NotLike,
    In,
    NotIn,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Binary { left: Operand, op: Op, right: Operand },
}

impl Expr {
    pub fn new_binary(left: &'static str, op: Op, right: Value) -> Expr {
        let op = match (&op, &right) {
            (Op::Eq, Value::Null) => Op::Is,
            (Op::Ne, Value::Null) => Op::IsNot,
            _ => op,
        };
        Expr::Binary { left: Operand::Column(left), op, right: Operand::Value(right) }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "expr_test.rs"]
mod tests;
