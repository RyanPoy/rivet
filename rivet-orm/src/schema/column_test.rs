use super::*;
use crate::ast::value::Value;
use crate::schema::column::Column;
use rivet_orm_macros::table;

pub fn binary(left: &'static str, op: Op, right: Value) -> Expr {
    Expr::Binary { left, op, right }
}

#[test]
pub fn test_eq_number() {
    assert_eq!(Column::<i32>::new("age").eq(20), binary("age", Op::Eq, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").eq(Some(20)), binary("age", Op::Eq, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").eq(None), binary("age", Op::Is, Value::Null));
}
#[test]
pub fn test_eq_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").eq(true),
        binary("has_children", Op::Eq, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").eq(Some(true)),
        binary("has_children", Op::Eq, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").eq(None),
        binary("has_children", Op::Is, Value::Null)
    );
}
#[test]
pub fn test_eq_string() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy".to_string()),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").eq(Some("Lucy".to_string())),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").eq(None::<String>),
        binary("username", Op::Is, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").eq("Lucy".to_string()),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").eq(Some("Lucy".to_string())),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").eq(None::<String>),
        binary("username", Op::Is, Value::Null)
    );
}
#[test]
pub fn test_eq_str_ref() {
    assert_eq!(
        Column::<String>::new("username").eq("Lucy"),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").eq(Some("Lucy")),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").eq(None::<String>),
        binary("username", Op::Is, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").eq("Lucy"),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").eq(Some("Lucy")),
        binary("username", Op::Eq, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").eq(None::<&str>),
        binary("username", Op::Is, Value::Null)
    );
}
#[test]
pub fn test_ne_number() {
    assert_eq!(Column::<i32>::new("age").ne(20), binary("age", Op::Ne, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").ne(20), binary("age", Op::Ne, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").ne(None), binary("age", Op::IsNot, Value::Null));
}
#[test]
pub fn test_ne_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").ne(true),
        binary("has_children", Op::Ne, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").ne(Some(true)),
        binary("has_children", Op::Ne, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").ne(None),
        binary("has_children", Op::IsNot, Value::Null)
    );
}
#[test]
pub fn test_ne_string() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy".to_string()),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").ne(Some("Lucy".to_string())),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").ne(None::<String>),
        binary("username", Op::IsNot, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").ne("Lucy".to_string()),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").ne(Some("Lucy".to_string())),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").ne(None::<String>),
        binary("username", Op::IsNot, Value::Null)
    );
}
#[test]
pub fn test_ne_str_ref() {
    assert_eq!(
        Column::<String>::new("username").ne("Lucy"),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").ne(Some("Lucy")),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").ne(None::<&str>),
        binary("username", Op::IsNot, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").ne("Lucy"),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").ne(Some("Lucy")),
        binary("username", Op::Ne, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").ne(None::<&str>),
        binary("username", Op::IsNot, Value::Null)
    );
}
#[test]
pub fn test_gt_number() {
    assert_eq!(Column::<i32>::new("age").gt(20), binary("age", Op::Gt, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").gt(Some(20)), binary("age", Op::Gt, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").gt(None), binary("age", Op::Gt, Value::Null));
}
#[test]
pub fn test_gt_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").gt(true),
        binary("has_children", Op::Gt, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").gt(true),
        binary("has_children", Op::Gt, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").gt(None),
        binary("has_children", Op::Gt, Value::Null)
    )
}
#[test]
pub fn test_gt_string() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy".to_string()),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gt("Lucy".to_string()),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gt(None::<String>),
        binary("username", Op::Gt, Value::Null)
    );

    assert_eq!(
        Column::<&str>::new("username").gt("Lucy".to_string()),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").gt(Some("Lucy".to_string())),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").gt(None::<String>),
        binary("username", Op::Gt, Value::Null)
    );
}
#[test]
pub fn test_gt_str_ref() {
    assert_eq!(
        Column::<String>::new("username").gt("Lucy"),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gt("Lucy"),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gt(None::<String>),
        binary("username", Op::Gt, Value::Null)
    );

    assert_eq!(
        Column::<&str>::new("username").gt("Lucy"),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").gt(Some("Lucy")),
        binary("username", Op::Gt, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").gt(None::<String>),
        binary("username", Op::Gt, Value::Null)
    );
}
#[test]
pub fn test_lt_number() {
    assert_eq!(Column::<i32>::new("age").lt(20), binary("age", Op::Lt, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").lt(Some(20)), binary("age", Op::Lt, Value::I32(20)));
    assert_eq!(Column::<i32>::new("age").lt(None), binary("age", Op::Lt, Value::Null));
}
#[test]
pub fn test_lt_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").lt(true),
        binary("has_children", Op::Lt, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").lt(Some(true)),
        binary("has_children", Op::Lt, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").lt(None),
        binary("has_children", Op::Lt, Value::Null)
    );
}
#[test]
pub fn test_lt_string() {
    assert_eq!(
        Column::<String>::new("username").lt("Lucy".to_string()),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lt(Some("Lucy".to_string())),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lt(None::<String>),
        binary("username", Op::Lt, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").lt("Lucy".to_string()),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lt(Some("Lucy".to_string())),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );

    assert_eq!(
        Column::<&str>::new("username").lt(None::<String>),
        binary("username", Op::Lt, Value::Null)
    );
}
#[test]
pub fn test_lt_str_ref() {
    assert_eq!(
        Column::<String>::new("username").lt("Lucy"),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lt(Some("Lucy")),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lt(None::<String>),
        binary("username", Op::Lt, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").lt("Lucy"),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lt(Some("Lucy")),
        binary("username", Op::Lt, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lt(None::<&str>),
        binary("username", Op::Lt, Value::Null)
    );
}
#[test]
pub fn test_gte_number() {
    assert_eq!(Column::<i32>::new("age").gte(20), binary("age", Op::Gte, Value::I32(20)));
    assert_eq!(
        Column::<i32>::new("age").gte(Some(20)),
        binary("age", Op::Gte, Value::I32(20))
    );
    assert_eq!(Column::<i32>::new("age").gte(None), binary("age", Op::Gte, Value::Null));
}
#[test]
pub fn test_gte_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").gte(true),
        binary("has_children", Op::Gte, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").gte(Some(true)),
        binary("has_children", Op::Gte, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").gte(None),
        binary("has_children", Op::Gte, Value::Null)
    );
}
#[test]
pub fn test_gte_string() {
    assert_eq!(
        Column::<String>::new("username").gte("Lucy".to_string()),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gte(Some("Lucy".to_string())),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gte(None::<String>),
        binary("username", Op::Gte, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").gte("Lucy".to_string()),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").gte(Some("Lucy".to_string())),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").gte(None::<String>),
        binary("username", Op::Gte, Value::Null)
    );
}
#[test]
pub fn test_gte_str_ref() {
    assert_eq!(
        Column::<String>::new("username").gte("Lucy"),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gte(Some("Lucy")),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").gte(None::<&str>),
        binary("username", Op::Gte, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").gte("Lucy"),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").gte(Some("Lucy")),
        binary("username", Op::Gte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").gte(None::<&str>),
        binary("username", Op::Gte, Value::Null)
    );
}

#[test]
pub fn test_lte_number() {
    assert_eq!(Column::<i32>::new("age").lte(20), binary("age", Op::Lte, Value::I32(20)));
    assert_eq!(
        Column::<i32>::new("age").lte(Some(20)),
        binary("age", Op::Lte, Value::I32(20))
    );
    assert_eq!(Column::<i32>::new("age").lte(None), binary("age", Op::Lte, Value::Null));
}

#[test]
pub fn test_lte_bool() {
    assert_eq!(
        Column::<bool>::new("has_children").lte(true),
        binary("has_children", Op::Lte, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").lte(Some(true)),
        binary("has_children", Op::Lte, Value::Bool(true))
    );
    assert_eq!(
        Column::<bool>::new("has_children").lte(None),
        binary("has_children", Op::Lte, Value::Null)
    );
}
#[test]
pub fn test_lte_string() {
    assert_eq!(
        Column::<String>::new("username").lte("Lucy".to_string()),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lte(Some("Lucy".to_string())),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lte(None::<String>),
        binary("username", Op::Lte, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").lte("Lucy".to_string()),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lte(Some("Lucy".to_string())),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lte(None::<String>),
        binary("username", Op::Lte, Value::Null)
    );
}
#[test]
pub fn test_lte_str_ref() {
    assert_eq!(
        Column::<String>::new("username").lte("Lucy"),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lte(Some("Lucy")),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").lte(None::<&str>),
        binary("username", Op::Lte, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").lte("Lucy"),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lte(Some("Lucy")),
        binary("username", Op::Lte, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").lte(None::<&str>),
        binary("username", Op::Lte, Value::Null)
    );
}
#[test]
pub fn test_like_string() {
    assert_eq!(
        Column::<String>::new("username").like("Lucy".to_string()),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").like(Some("Lucy".to_string())),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").like(None::<String>),
        binary("username", Op::Like, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").like("Lucy".to_string()),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").like(Some("Lucy".to_string())),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").like(None::<String>),
        binary("username", Op::Like, Value::Null)
    );
}
#[test]
pub fn test_like_str_ref() {
    assert_eq!(
        Column::<String>::new("username").like("Lucy"),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").like(Some("Lucy")),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").like(None::<&str>),
        binary("username", Op::Like, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").like("Lucy"),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").like(Some("Lucy")),
        binary("username", Op::Like, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").like(None::<&str>),
        binary("username", Op::Like, Value::Null)
    );
}
#[test]
pub fn test_not_like_string() {
    assert_eq!(
        Column::<String>::new("username").not_like("Lucy".to_string()),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").not_like(Some("Lucy".to_string())),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").not_like(None::<String>),
        binary("username", Op::NotLike, Value::Null)
    );
    assert_eq!(
        Column::<&str>::new("username").not_like("Lucy".to_string()),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").not_like(Some("Lucy".to_string())),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").not_like(None::<String>),
        binary("username", Op::NotLike, Value::Null)
    );
}
#[test]
pub fn test_not_like_str_ref() {
    assert_eq!(
        Column::<String>::new("username").not_like("Lucy"),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").not_like(Some("Lucy")),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<String>::new("username").not_like(None::<&str>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
    assert_eq!(
        Column::<&str>::new("username").not_like("Lucy"),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").not_like(Some("Lucy")),
        binary("username", Op::NotLike, Value::String("Lucy".to_string()))
    );
    assert_eq!(
        Column::<&str>::new("username").not_like(None::<&str>),
        Expr::Binary { left: "username", op: Op::NotLike, right: Value::Null }
    );
}
//
// #[test]
// fn test_column_define() {
//     #[table]
//     struct User {
//         id: i32,
//
//         #[col(username)]
//         name: String,
//
//         age: u32,
//         nick_name: Option<String>,
//         checked: bool,
//
//         #[no_col]
//         temp: String,
//     };
//
//     let user = User {
//         id: 1,
//         name: "Luly".to_string(),
//         age: 30,
//         nick_name: None,
//         checked: true,
//         temp: "fuck".to_string(),
//     };
//     assert_eq!(User::id, Column::<i32>::new("id"));
//     assert_eq!(User::name, Column::<String>::new("username"));
//     assert_eq!(User::age, Column::<u32>::new("age"));
//     // assert_eq!(User::nick_name, Column::<Option<String>>::new("nick_name"));
//     assert_eq!(User::checked, Column::<bool>::new("checked"));
//
//     assert_eq!(user.id, 1);
// }
