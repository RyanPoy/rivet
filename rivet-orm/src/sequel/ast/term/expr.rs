use crate::sequel::ast::{Operand, Value};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Binary { left: Operand, op: Op, right: Operand },
    And { left: Box<Expr>, right: Box<Expr> },
    Or { left: Box<Expr>, right: Box<Expr> },
    Not { expr: Box<Expr> },
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

    pub fn and(self, other: Expr) -> Expr {
        Expr::And { left: Box::new(self), right: Box::new(other) }
    }

    pub fn or(self, other: Expr) -> Expr {
        Expr::Or { left: Box::new(self), right: Box::new(other) }
    }

    pub fn not(self) -> Expr {
        Expr::Not { expr: Box::new(self) }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "expr_test.rs"]
mod tests;
