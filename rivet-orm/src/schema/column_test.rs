use super::*;
use crate::ast::value::Value;

#[test]
pub fn test_eq() {
    assert_eq!(
        Column::<i32>::new("age").eq(20),
        Expr::Binary {
            left: "age",
            op: Op::Eq,
            right: Value::I32(Some(20)),
        }
    );
}

#[test]
pub fn test_ne() {
    assert_eq!(
        Column::<i32>::new("age").ne(20),
        Expr::Binary {
            left: "age",
            op: Op::Ne,
            right: Value::I32(Some(20)),
        }
    );
}

#[test]
pub fn test_gt() {
    assert_eq!(
        Column::<i32>::new("age").gt(20),
        Expr::Binary {
            left: "age",
            op: Op::Gt,
            right: Value::I32(Some(20)),
        }
    );
}

#[test]
pub fn test_gte() {
    assert_eq!(
        Column::<i32>::new("age").gte(20),
        Expr::Binary {
            left: "age",
            op: Op::Gte,
            right: Value::I32(Some(20)),
        }
    );
}

#[test]
pub fn test_lt() {
    assert_eq!(
        Column::<i32>::new("age").lt(20),
        Expr::Binary {
            left: "age",
            op: Op::Lt,
            right: Value::I32(Some(20)),
        }
    );
}

#[test]
pub fn test_lte() {
    assert_eq!(
        Column::<i32>::new("age").lte(20),
        Expr::Binary {
            left: "age",
            op: Op::Lte,
            right: Value::I32(Some(20)),
        }
    );
}
