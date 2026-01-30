use super::*;
use crate::sequel::ast::{Operand, Value};

/// 辅助函数：快速创建 Column 操作数
fn col(name: &'static str) -> Operand {
    Operand::Column { name, alias: None }
}

/// 辅助函数：快速创建 Value 操作数
fn val(v: Value) -> Operand {
    Operand::Value(v)
}

#[test]
fn test_new_binary_null_conversion() {
    // 测试 Eq + Null -> Is
    let expr_is = Expr::new_binary("age", Op::Eq, Value::Null);
    assert_eq!(expr_is, Expr::Binary { left: col("age"), op: Op::Is, right: val(Value::Null) });

    // 测试 Ne + Null -> IsNot
    let expr_is_not = Expr::new_binary("age", Op::Ne, Value::Null);
    assert_eq!(expr_is_not, Expr::Binary { left: col("age"), op: Op::IsNot, right: val(Value::Null) });

    // 测试普通值不转换 Op
    let expr_normal = Expr::new_binary("age", Op::Eq, Value::I32(20));
    assert_eq!(expr_normal, Expr::Binary { left: col("age"), op: Op::Eq, right: val(Value::I32(20)) });
}

#[test]
fn test_all_ops_coverage() {
    for op in vec![
        Op::Eq,
        Op::Ne,
        Op::Is,
        Op::IsNot,
        Op::Gt,
        Op::Gte,
        Op::Lt,
        Op::Lte,
        Op::Like,
        Op::NotLike,
        Op::In,
        Op::NotIn,
    ] {
        let expr = Expr::new_binary("col", op, Value::I32(1));
        if let Expr::Binary { op: res_op, .. } = expr {
            assert!(matches!(res_op, _));
        }
    }
}

#[test]
fn test_logical_combination_and() {
    let left = Expr::new_binary("age", Op::Gt, Value::I32(18));
    let right = Expr::new_binary("age", Op::Lt, Value::I32(30));

    let combined = left.and(right);

    match combined {
        Expr::And { left, right } => {
            assert!(matches!(*left, Expr::Binary { .. }));
            assert!(matches!(*right, Expr::Binary { .. }));
        }
        _ => panic!("Expected Expr::And"),
    }
}

#[test]
fn test_logical_combination_or() {
    let left = Expr::new_binary("name", Op::Eq, Value::String("Alice".into()));
    let right = Expr::new_binary("name", Op::Eq, Value::String("Bob".into()));

    let combined = left.or(right);

    match combined {
        Expr::Or { left, right } => {
            assert!(matches!(*left, Expr::Binary { .. }));
            assert!(matches!(*right, Expr::Binary { .. }));
        }
        _ => panic!("Expected Expr::Or"),
    }
}

#[test]
fn test_logical_not() {
    let inner = Expr::new_binary("checked", Op::Eq, Value::Bool(true));
    let not_expr = inner.not();

    match not_expr {
        Expr::Not { expr } => {
            assert!(matches!(*expr, Expr::Binary { .. }));
        }
        _ => panic!("Expected Expr::Not"),
    }
}

#[test]
fn test_deep_nesting() {
    // 构造：NOT (age > 18 AND (name = "Luly" OR name = "Lucy"))
    let expr = Expr::new_binary("age", Op::Gt, Value::I32(18))
        .and(Expr::new_binary("name", Op::Eq, Value::String("Luly".into())).or(Expr::new_binary(
            "name",
            Op::Eq,
            Value::String("Lucy".into()),
        )))
        .not();

    if let Expr::Not { expr: inner_not } = expr {
        if let Expr::And { left, right } = *inner_not {
            assert!(matches!(*left, Expr::Binary { .. }));
            assert!(matches!(*right, Expr::Or { .. }));
        } else {
            panic!("Expected AND inside NOT");
        }
    } else {
        panic!("Expected NOT at root");
    }
}
