use super::*;
use crate::ast::value::Value;
use crate::schema::column::Column;
use rivet_orm_macros::table;

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

        gender: &'static str,

        #[no_col]
        temp: String,
    };

    let user = User {
        id: 1,
        name: "Luly".to_string(),
        age: 30,
        nick_name: None,
        gender: "male",
        checked: true,
        temp: "fuck".to_string(),
    };
    assert_eq!(User::id, Column::<i32>::new("id"));
    assert_eq!(User::name, Column::<String>::new("username"));
    assert_eq!(User::age, Column::<u32>::new("age"));
    assert_eq!(User::nick_name, Column::<String>::new("nick_name"));
    assert_eq!(User::checked, Column::<bool>::new("checked"));
    assert_eq!(User::gender, Column::<String>::new("gender"));
    assert_eq!(user.id, 1);
}

macro_rules! assert_exprs_eq {
    ($($expr:expr => {$left:expr, $op:path, $right:expr}),* $(,)?) => {
        $(
            assert_eq!(
                $expr,
                Expr::Binary { left: $left, op: $op, right: $right, }
            );
        )*
    };
}
#[test]
pub fn test_eq() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.eq(20) => {"age", Op::Eq, Value::I32(20)},
        age.eq(Some(20)) => {"age", Op::Eq, Value::I32(20)},
        age.eq(None) => {"age", Op::Is, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.eq(true) => {"has_children", Op::Eq, Value::Bool(true)},
        has_children.eq(Some(true)) => {"has_children", Op::Eq, Value::Bool(true)},
        has_children.eq(None) => {"has_children", Op::Is, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.eq("Lucy".to_string()) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        username.eq(Some("Lucy".to_string())) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        username.eq(None::<String>) => {"username", Op::Is, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        username.eq("Lucy") => {"username", Op::Eq, Value::String("Lucy".to_string())},
        username.eq(Some("Lucy")) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        username.eq(None::<&str>) => {"username", Op::Is, Value::Null},
    );
}
#[test]
pub fn test_ne() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.ne(20) => {"age", Op::Ne, Value::I32(20)},
        age.ne(Some(20)) => {"age", Op::Ne, Value::I32(20)},
        age.ne(None) => {"age", Op::IsNot, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.ne(true) => {"has_children", Op::Ne, Value::Bool(true)},
        has_children.ne(Some(true)) => {"has_children", Op::Ne, Value::Bool(true)},
        has_children.ne(None) => {"has_children", Op::IsNot, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.ne("Lucy".to_string()) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        username.ne(Some("Lucy".to_string())) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        username.ne(None::<String>) => {"username", Op::IsNot, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        username.ne("Lucy") => {"username", Op::Ne, Value::String("Lucy".to_string())},
        username.ne(Some("Lucy")) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        username.ne(None::<&str>) => {"username", Op::IsNot, Value::Null},
    );
}
#[test]
pub fn test_gt() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.gt(20) => {"age", Op::Gt, Value::I32(20)},
        age.gt(Some(20)) => {"age", Op::Gt, Value::I32(20)},
        age.gt(None) => {"age", Op::Gt, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.gt(true) => {"has_children", Op::Gt, Value::Bool(true)},
        has_children.gt(Some(true)) => {"has_children", Op::Gt, Value::Bool(true)},
        has_children.gt(None) => {"has_children", Op::Gt, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.gt("Lucy".to_string()) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        username.gt(Some("Lucy".to_string())) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        username.gt(None::<String>) => {"username", Op::Gt, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        username.gt("Lucy") => {"username", Op::Gt, Value::String("Lucy".to_string())},
        username.gt(Some("Lucy")) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        username.gt(None::<&str>) => {"username", Op::Gt, Value::Null},
    );
}
#[test]
pub fn test_lt() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.lt(20) => {"age", Op::Lt, Value::I32(20)},
        age.lt(Some(20)) => {"age", Op::Lt, Value::I32(20)},
        age.lt(None) => {"age", Op::Lt, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.lt(true) => {"has_children", Op::Lt, Value::Bool(true)},
        has_children.lt(Some(true)) => {"has_children", Op::Lt, Value::Bool(true)},
        has_children.lt(None) => {"has_children", Op::Lt, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.lt("Lucy".to_string()) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        username.lt(Some("Lucy".to_string())) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        username.lt(None::<String>) => {"username", Op::Lt, Value::Null},
    );

    // for &str\
    assert_exprs_eq!(
        username.lt("Lucy") => {"username", Op::Lt, Value::String("Lucy".to_string())},
        username.lt(Some("Lucy")) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        username.lt(None::<&str>) => {"username", Op::Lt, Value::Null},
    );
}
#[test]
pub fn test_gte() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.gte(20) => {"age", Op::Gte, Value::I32(20)},
        age.gte(Some(20)) => {"age", Op::Gte, Value::I32(20)},
        age.gte(None) => {"age", Op::Gte, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.gte(true) => {"has_children", Op::Gte, Value::Bool(true)},
        has_children.gte(Some(true)) => {"has_children", Op::Gte, Value::Bool(true)},
        has_children.gte(None) => {"has_children", Op::Gte, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.gte("Lucy".to_string()) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        username.gte(Some("Lucy".to_string())) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        username.gte(None::<String>) => {"username", Op::Gte, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        username.gte("Lucy") => {"username", Op::Gte, Value::String("Lucy".to_string())},
        username.gte(Some("Lucy")) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        username.gte(None::<&str>) => {"username", Op::Gte, Value::Null},
    );
}

#[test]
pub fn test_lte() {
    // for number
    let age = Column::<i32>::new("age");
    assert_exprs_eq!(
        age.lte(20) => {"age", Op::Lte, Value::I32(20)},
        age.lte(Some(20)) => {"age", Op::Lte, Value::I32(20)},
        age.lte(None) => {"age", Op::Lte, Value::Null},
    );

    // for bool
    let has_children = Column::<bool>::new("has_children");
    assert_exprs_eq!(
        has_children.lte(true) => {"has_children", Op::Lte, Value::Bool(true)},
        has_children.lte(Some(true)) => {"has_children", Op::Lte, Value::Bool(true)},
        has_children.lte(None) => {"has_children", Op::Lte, Value::Null},
    );

    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.lte("Lucy".to_string()) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        username.lte(Some("Lucy".to_string())) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        username.lte(None::<String>) => {"username", Op::Lte, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        username.lte("Lucy") => {"username", Op::Lte, Value::String("Lucy".to_string())},
        username.lte(Some("Lucy")) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        username.lte(None::<&str>) => {"username", Op::Lte, Value::Null},
    );
}
#[test]
pub fn test_like() {
    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.like("Lucy".to_string()) => {"username", Op::Like, Value::String("Lucy".to_string())},
        username.like(Some("Lucy".to_string())) => {"username", Op::Like, Value::String("Lucy".to_string())},
        username.like(None::<String>) => {"username", Op::Like, Value::Null},
    );
    // for &str
    assert_exprs_eq!(
        username.like("Lucy") => {"username", Op::Like, Value::String("Lucy".to_string())},
        username.like(Some("Lucy")) => {"username", Op::Like, Value::String("Lucy".to_string())},
        username.like(None::<&str>) => {"username", Op::Like, Value::Null},
    );
}
#[test]
pub fn test_not_like() {
    // for String
    let username = Column::<String>::new("username");
    assert_exprs_eq!(
        username.not_like("Lucy".to_string()) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        username.not_like(Some("Lucy".to_string())) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        username.not_like(None::<String>) => {"username", Op::NotLike, Value::Null},
    );
    // for &str
    assert_exprs_eq!(
        username.not_like("Lucy") => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        username.not_like(Some("Lucy")) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        username.not_like(None::<&str>) => {"username", Op::NotLike, Value::Null},
    );
}
