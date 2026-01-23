use super::*;
use crate::ast::value::Value;
use crate::schema::column::Column;
use rivet_orm_macros::table;

#[test]
pub fn test_eq_number() {
    assert_eq!(Column::<i32>::new("age").eq(20), Expr::Binary { left: "age", op: Op::Eq, right: Value::I32(Some(20)) });

    assert_eq!(
        Column::<Option<i32>>::new("age").eq(20),
        Expr::Binary { left: "age", op: Op::Eq, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").eq(None),
        Expr::Binary { left: "age", op: Op::Is, right: Value::Null }
    );
}
#[test]
pub fn test_eq_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").eq(true),
        Expr::Binary { left: "has_children", op: Op::Eq, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").eq(true),
        Expr::Binary { left: "has_children", op: Op::Eq, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").eq(None),
        Expr::Binary { left: "has_children", op: Op::Is, right: Value::Null }
    );
}
#[test]
pub fn test_eq_string() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").eq("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").eq("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq(None::<String>),
        Expr::Binary { left: "username", op: Op::Is, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").eq(None::<String>),
        Expr::Binary { left: "username", op: Op::Is, right: Value::Null }
    );
}
#[test]
pub fn test_eq_str_ref() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy"),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").eq("Lucy"),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq("Lucy"),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").eq("Lucy"),
        Expr::Binary { left: "username", op: Op::Eq, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").eq(None::<&str>),
        Expr::Binary { left: "username", op: Op::Is, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").eq(None::<&str>),
        Expr::Binary { left: "username", op: Op::Is, right: Value::Null }
    );
}
#[test]
pub fn test_ne_number() {
    assert_eq!(Column::<i32>::new("age").ne(20), Expr::Binary { left: "age", op: Op::Ne, right: Value::I32(Some(20)) });

    assert_eq!(
        Column::<Option<i32>>::new("age").ne(20),
        Expr::Binary { left: "age", op: Op::Ne, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").ne(None),
        Expr::Binary { left: "age", op: Op::IsNot, right: Value::Null }
    );
}
#[test]
pub fn test_ne_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").ne(true),
        Expr::Binary { left: "has_children", op: Op::Ne, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").ne(true),
        Expr::Binary { left: "has_children", op: Op::Ne, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").ne(None),
        Expr::Binary { left: "has_children", op: Op::IsNot, right: Value::Null }
    );
}
#[test]
pub fn test_ne_string() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").ne("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").ne("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").ne("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").ne(None::<String>),
        Expr::Binary { left: "username", op: Op::IsNot, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").ne(None::<String>),
        Expr::Binary { left: "username", op: Op::IsNot, right: Value::Null }
    );
}
#[test]
pub fn test_ne_str_ref() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy"),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").ne("Lucy"),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );

    assert_eq!(
        Column::<Option<String>>::new("username").ne("Lucy"),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").ne("Lucy"),
        Expr::Binary { left: "username", op: Op::Ne, right: Value::String(Some("Lucy".to_string())) }
    );

    assert_eq!(
        Column::<Option<String>>::new("username").ne(None::<&str>),
        Expr::Binary { left: "username", op: Op::IsNot, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").ne(None::<&str>),
        Expr::Binary { left: "username", op: Op::IsNot, right: Value::Null }
    );
}
#[test]
pub fn test_gt_number() {
    assert_eq!(Column::<i32>::new("age").gt(20), Expr::Binary { left: "age", op: Op::Gt, right: Value::I32(Some(20)) });

    assert_eq!(
        Column::<Option<i32>>::new("age").gt(20),
        Expr::Binary { left: "age", op: Op::Gt, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").gt(None),
        Expr::Binary { left: "age", op: Op::Gt, right: Value::Null }
    );
}
#[test]
pub fn test_gt_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").gt(true),
        Expr::Binary { left: "has_children", op: Op::Gt, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gt(true),
        Expr::Binary { left: "has_children", op: Op::Gt, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gt(None),
        Expr::Binary { left: "has_children", op: Op::Gt, right: Value::Null }
    );
}
#[test]
pub fn test_gt_string() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").gt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt(None::<String>),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gt(None::<String>),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::Null }
    );
}
#[test]
pub fn test_gt_str_ref() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy"),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").gt("Lucy"),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt("Lucy"),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gt("Lucy"),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gt(None::<&str>),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gt(None::<&str>),
        Expr::Binary { left: "username", op: Op::Gt, right: Value::Null }
    );
}
#[test]
pub fn test_lt_number() {
    assert_eq!(Column::<i32>::new("age").lt(20), Expr::Binary { left: "age", op: Op::Lt, right: Value::I32(Some(20)) });

    assert_eq!(
        Column::<Option<i32>>::new("age").lt(20),
        Expr::Binary { left: "age", op: Op::Lt, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").lt(None),
        Expr::Binary { left: "age", op: Op::Lt, right: Value::Null }
    );
}
#[test]
pub fn test_lt_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").lt(true),
        Expr::Binary { left: "has_children", op: Op::Lt, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").lt(true),
        Expr::Binary { left: "has_children", op: Op::Lt, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").lt(None),
        Expr::Binary { left: "has_children", op: Op::Lt, right: Value::Null }
    );
}
#[test]
pub fn test_lt_string() {
    assert_eq!(
        Column::<String>::new("username").lt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").lt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lt("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lt(None::<String>),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lt(None::<String>),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::Null }
    );
}
#[test]
pub fn test_lt_str_ref() {
    assert_eq!(
        Column::<String>::new("username").lt("Lucy"),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").lt("Lucy"),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lt("Lucy"),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lt("Lucy"),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lt(None::<&str>),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lt(None::<&str>),
        Expr::Binary { left: "username", op: Op::Lt, right: Value::Null }
    );
}
#[test]
pub fn test_gte_number() {
    assert_eq!(
        Column::<i32>::new("age").gte(20),
        Expr::Binary { left: "age", op: Op::Gte, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").gte(20),
        Expr::Binary { left: "age", op: Op::Gte, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").gte(None),
        Expr::Binary { left: "age", op: Op::Gte, right: Value::Null }
    );
}
#[test]
pub fn test_gte_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").gte(true),
        Expr::Binary { left: "has_children", op: Op::Gte, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gte(true),
        Expr::Binary { left: "has_children", op: Op::Gte, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").gte(None),
        Expr::Binary { left: "has_children", op: Op::Gte, right: Value::Null }
    );
}
#[test]
pub fn test_gte_string() {
    assert_eq!(
        Column::<String>::new("username").gte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").gte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gte(None::<String>),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gte(None::<String>),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::Null }
    );
}
#[test]
pub fn test_gte_str_ref() {
    assert_eq!(
        Column::<String>::new("username").gte("Lucy"),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").gte("Lucy"),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gte("Lucy"),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gte("Lucy"),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").gte(None::<&str>),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").gte(None::<&str>),
        Expr::Binary { left: "username", op: Op::Gte, right: Value::Null }
    );
}

#[test]
pub fn test_lte_number() {
    assert_eq!(
        Column::<i32>::new("age").lte(20),
        Expr::Binary { left: "age", op: Op::Lte, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").lte(20),
        Expr::Binary { left: "age", op: Op::Lte, right: Value::I32(Some(20)) }
    );

    assert_eq!(
        Column::<Option<i32>>::new("age").lte(None),
        Expr::Binary { left: "age", op: Op::Lte, right: Value::Null }
    );
}
#[test]
pub fn test_lte_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").lte(true),
        Expr::Binary { left: "has_children", op: Op::Lte, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").lte(true),
        Expr::Binary { left: "has_children", op: Op::Lte, right: Value::Bool(Some(true)) }
    );
    assert_eq!(
        Column::<Option<bool>>::new("has_children").lte(None),
        Expr::Binary { left: "has_children", op: Op::Lte, right: Value::Null }
    );
}
#[test]
pub fn test_lte_string() {
    assert_eq!(
        Column::<String>::new("username").lte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").lte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lte("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lte(None::<String>),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lte(None::<String>),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::Null }
    );
}
#[test]
pub fn test_lte_str_ref() {
    assert_eq!(
        Column::<String>::new("username").lte("Lucy"),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").lte("Lucy"),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lte("Lucy"),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lte("Lucy"),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").lte(None::<&str>),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").lte(None::<&str>),
        Expr::Binary { left: "username", op: Op::Lte, right: Value::Null }
    );
}
#[test]
pub fn test_like_string() {
    assert_eq!(
        Column::<String>::new("username").like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").like(None::<String>),
        Expr::Binary { left: "username", op: Op::Like, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").like(None::<String>),
        Expr::Binary { left: "username", op: Op::Like, right: Value::Null }
    );
}
#[test]
pub fn test_like_str_ref() {
    assert_eq!(
        Column::<String>::new("username").like("Lucy"),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").like("Lucy"),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").like("Lucy"),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").like("Lucy"),
        Expr::Binary { left: "username", op: Op::Like, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").like(None::<&str>),
        Expr::Binary { left: "username", op: Op::Like, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").like(None::<&str>),
        Expr::Binary { left: "username", op: Op::Like, right: Value::Null }
    );
}
#[test]
pub fn test_not_like_string() {
    assert_eq!(
        Column::<String>::new("username").not_like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").not_like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").not_like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").not_like("Lucy".to_string()),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").not_like(None::<String>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").not_like(None::<String>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
}
#[test]
pub fn test_not_like_str_ref() {
    assert_eq!(
        Column::<String>::new("username").not_like("Lucy"),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<&str>::new("username").not_like("Lucy"),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").not_like("Lucy"),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").not_like("Lucy"),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::String(Some("Lucy".to_string())) }
    );
    assert_eq!(
        Column::<Option<String>>::new("username").not_like(None::<&str>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
    assert_eq!(
        Column::<Option<&str>>::new("username").not_like(None::<&str>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
}

#[test]
fn test_column_define() {
    #[table]
    struct User {
        id: i32,

        #[col(username)]
        name: String,

        age: u32,
        nick_name: Option<String>,
        checked: bool,

        #[no_col]
        temp: String,
    };

    let user = User { id: 1, name: "Luly".to_string(), age: 30, nick_name: None, checked: true, temp: "fuck".to_string() };
    assert_eq!(User::id, Column::<i32>::new("id"));
    assert_eq!(User::name, Column::<String>::new("username"));
    assert_eq!(User::age, Column::<u32>::new("age"));
    assert_eq!(User::nick_name, Column::<Option<String>>::new("nick_name"));
    assert_eq!(User::checked, Column::<bool>::new("checked"));

    assert_eq!(user.id, 1);
}
