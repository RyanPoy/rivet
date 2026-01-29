use crate::sequel::ast::{Operand, Value};
use crate::sequel::build::Binder;

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

impl Op {
    fn to_string(self) -> &'static str {
        match self {
            Op::Eq => "=",
            Op::Ne => "<>",
            Op::Is => "IS",
            Op::IsNot => "IS NOT",
            Op::Gt => ">",
            Op::Gte => ">=",
            Op::Lt => "<",
            Op::Lte => "<=",
            Op::Like => "LIKE",
            Op::NotLike => "NOT LIKE",
            Op::In => "IN",
            Op::NotIn => "NOT IN",
        }
    }
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

    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Expr::Binary { left, op, right } => {
                format!("{} {} {}", left.build(binder), op.to_string(), right.build(binder))
            }
            Expr::And { left, right } => {
                format!("({} AND {})", left.build(binder), right.build(binder))
            }
            Expr::Or { left, right } => {
                format!("({} OR {})", left.build(binder), right.build(binder))
            }
            Expr::Not { expr } => {
                format!("NOT ({})", expr.build(binder))
            }
        }
    }
}

/// 测试模块。
/// Test module.
#[cfg(test)]
#[path = "expr_test.rs"]
mod tests;
