use super::*;
use crate::ast::value::Value;

#[test]
pub fn test_eq_number() {
    assert_eq!(
        Column::<i32>::new("age").eq(20),
        Expr::Binary {
            left: "age",
            op: Op::Eq,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").eq(20),
        Expr::Binary {
            left: "age",
            op: Op::Eq,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").eq(None),
        Expr::Binary {
            left: "age",
            op: Op::Is,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_eq_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").eq(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Eq,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").eq(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Eq,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").eq(None),
        Expr::Binary {
            left: "has_children",
            op: Op::Is,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_eq_string() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Eq,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Eq,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq(None::<String>),
        Expr::Binary {
            left: "username",
            op: Op::Is,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_eq_str_ref() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Eq,
            right: Value::String(Some("Lucy".to_string())),
        }
    );

    assert_eq!(
        Column::<Option<String>>::new("username").eq("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Eq,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq(None::<&str>),
        Expr::Binary {
            left: "username",
            op: Op::Is,
            right: Value::Null,
        }
    );
}

#[test]
pub fn test_ne_number() {
    assert_eq!(
        Column::<i32>::new("age").ne(20),
        Expr::Binary {
            left: "age",
            op: Op::Ne,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").ne(20),
        Expr::Binary {
            left: "age",
            op: Op::Ne,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").ne(None),
        Expr::Binary {
            left: "age",
            op: Op::IsNot,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_ne_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").ne(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Ne,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").ne(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Ne,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").ne(None),
        Expr::Binary {
            left: "has_children",
            op: Op::IsNot,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_ne_string() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Ne,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").ne("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Ne,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").ne(None::<String>),
        Expr::Binary {
            left: "username",
            op: Op::IsNot,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_ne_str_ref() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Ne,
            right: Value::String(Some("Lucy".to_string())),
        }
    );

    assert_eq!(
        Column::<Option<String>>::new("username").ne("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Ne,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").ne(None::<&str>),
        Expr::Binary {
            left: "username",
            op: Op::IsNot,
            right: Value::Null,
        }
    );
}


#[test]
pub fn test_gt_number() {
    assert_eq!(
        Column::<i32>::new("age").gt(20),
        Expr::Binary {
            left: "age",
            op: Op::Gt,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").gt(20),
        Expr::Binary {
            left: "age",
            op: Op::Gt,
            right: Value::I32(Some(20)),
        }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").gt(None),
        Expr::Binary {
            left: "age",
            op: Op::Gt,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_gt_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").gt(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Gt,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gt(true),
        Expr::Binary {
            left: "has_children",
            op: Op::Gt,
            right: Value::Bool(Some(true)),
        }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gt(None),
        Expr::Binary {
            left: "has_children",
            op: Op::Gt,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_gt_string() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt("Lucy".to_string()),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt(None::<String>),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::Null,
        }
    );
}
#[test]
pub fn test_gt_str_ref() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::String(Some("Lucy".to_string())),
        }
    );

    assert_eq!(
        Column::<Option<String>>::new("username").gt("Lucy"),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::String(Some("Lucy".to_string())),
        }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt(None::<&str>),
        Expr::Binary {
            left: "username",
            op: Op::Gt,
            right: Value::Null,
        }
    );
}

// #[test]
// pub fn test_gte() {
//     assert_eq!(
//         Column::<i32>::new("age").gte(20),
//         Expr::Binary {
//             left: "age",
//             op: Op::Gte,
//             right: Value::I32(Some(20)),
//         }
//     );
// }
//
// #[test]
// pub fn test_lt() {
//     assert_eq!(
//         Column::<i32>::new("age").lt(20),
//         Expr::Binary {
//             left: "age",
//             op: Op::Lt,
//             right: Value::I32(Some(20)),
//         }
//     );
// }
//
// #[test]
// pub fn test_lte() {
//     assert_eq!(
//         Column::<i32>::new("age").lte(20),
//         Expr::Binary {
//             left: "age",
//             op: Op::Lte,
//             right: Value::I32(Some(20)),
//         }
//     );
// }
