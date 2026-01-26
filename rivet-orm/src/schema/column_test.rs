use super::*;
use crate::ast::value::Value;
use crate::schema::column::Column;
use rivet_orm_macros::table;

#[allow(non_upper_case_globals)]
mod setup {
    use crate::orm::Column;
    pub const age: Column<i32> = Column::new("age");
    pub const has_children: Column<bool> = Column::new("has_children");
    pub const username: Column<String> = Column::new("username");
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
    assert_exprs_eq!(
        setup::age.eq(20) => {"age", Op::Eq, Value::I32(20)},
        setup::age.eq(Some(20)) => {"age", Op::Eq, Value::I32(20)},
        setup::age.eq(None) => {"age", Op::Is, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.eq(true) => {"has_children", Op::Eq, Value::Bool(true)},
        setup::has_children.eq(Some(true)) => {"has_children", Op::Eq, Value::Bool(true)},
        setup::has_children.eq(None) => {"has_children", Op::Is, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.eq("Lucy".to_string()) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        setup::username.eq(Some("Lucy".to_string())) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        setup::username.eq(None::<String>) => {"username", Op::Is, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        setup::username.eq("Lucy") => {"username", Op::Eq, Value::String("Lucy".to_string())},
        setup::username.eq(Some("Lucy")) => {"username", Op::Eq, Value::String("Lucy".to_string())},
        setup::username.eq(None::<&str>) => {"username", Op::Is, Value::Null},
    );
}
#[test]
pub fn test_ne() {
    // for number
    assert_exprs_eq!(
        setup::age.ne(20) => {"age", Op::Ne, Value::I32(20)},
        setup::age.ne(Some(20)) => {"age", Op::Ne, Value::I32(20)},
        setup::age.ne(None) => {"age", Op::IsNot, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.ne(true) => {"has_children", Op::Ne, Value::Bool(true)},
        setup::has_children.ne(Some(true)) => {"has_children", Op::Ne, Value::Bool(true)},
        setup::has_children.ne(None) => {"has_children", Op::IsNot, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.ne("Lucy".to_string()) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        setup::username.ne(Some("Lucy".to_string())) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        setup::username.ne(None::<String>) => {"username", Op::IsNot, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        setup::username.ne("Lucy") => {"username", Op::Ne, Value::String("Lucy".to_string())},
        setup::username.ne(Some("Lucy")) => {"username", Op::Ne, Value::String("Lucy".to_string())},
        setup::username.ne(None::<&str>) => {"username", Op::IsNot, Value::Null},
    );
}
#[test]
pub fn test_gt() {
    // for number
    assert_exprs_eq!(
        setup::age.gt(20) => {"age", Op::Gt, Value::I32(20)},
        setup::age.gt(Some(20)) => {"age", Op::Gt, Value::I32(20)},
        setup::age.gt(None) => {"age", Op::Gt, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.gt(true) => {"has_children", Op::Gt, Value::Bool(true)},
        setup::has_children.gt(Some(true)) => {"has_children", Op::Gt, Value::Bool(true)},
        setup::has_children.gt(None) => {"has_children", Op::Gt, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.gt("Lucy".to_string()) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        setup::username.gt(Some("Lucy".to_string())) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        setup::username.gt(None::<String>) => {"username", Op::Gt, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        setup::username.gt("Lucy") => {"username", Op::Gt, Value::String("Lucy".to_string())},
        setup::username.gt(Some("Lucy")) => {"username", Op::Gt, Value::String("Lucy".to_string())},
        setup::username.gt(None::<&str>) => {"username", Op::Gt, Value::Null},
    );
}
#[test]
pub fn test_lt() {
    // for number
    assert_exprs_eq!(
        setup::age.lt(20) => {"age", Op::Lt, Value::I32(20)},
        setup::age.lt(Some(20)) => {"age", Op::Lt, Value::I32(20)},
        setup::age.lt(None) => {"age", Op::Lt, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.lt(true) => {"has_children", Op::Lt, Value::Bool(true)},
        setup::has_children.lt(Some(true)) => {"has_children", Op::Lt, Value::Bool(true)},
        setup::has_children.lt(None) => {"has_children", Op::Lt, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.lt("Lucy".to_string()) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        setup::username.lt(Some("Lucy".to_string())) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        setup::username.lt(None::<String>) => {"username", Op::Lt, Value::Null},
    );

    // for &str\
    assert_exprs_eq!(
        setup::username.lt("Lucy") => {"username", Op::Lt, Value::String("Lucy".to_string())},
        setup::username.lt(Some("Lucy")) => {"username", Op::Lt, Value::String("Lucy".to_string())},
        setup::username.lt(None::<&str>) => {"username", Op::Lt, Value::Null},
    );
}
#[test]
pub fn test_gte() {
    // for number
    assert_exprs_eq!(
        setup::age.gte(20) => {"age", Op::Gte, Value::I32(20)},
        setup::age.gte(Some(20)) => {"age", Op::Gte, Value::I32(20)},
        setup::age.gte(None) => {"age", Op::Gte, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.gte(true) => {"has_children", Op::Gte, Value::Bool(true)},
        setup::has_children.gte(Some(true)) => {"has_children", Op::Gte, Value::Bool(true)},
        setup::has_children.gte(None) => {"has_children", Op::Gte, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.gte("Lucy".to_string()) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        setup::username.gte(Some("Lucy".to_string())) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        setup::username.gte(None::<String>) => {"username", Op::Gte, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        setup::username.gte("Lucy") => {"username", Op::Gte, Value::String("Lucy".to_string())},
        setup::username.gte(Some("Lucy")) => {"username", Op::Gte, Value::String("Lucy".to_string())},
        setup::username.gte(None::<&str>) => {"username", Op::Gte, Value::Null},
    );
}

#[test]
pub fn test_lte() {
    // for number
    assert_exprs_eq!(
        setup::age.lte(20) => {"age", Op::Lte, Value::I32(20)},
        setup::age.lte(Some(20)) => {"age", Op::Lte, Value::I32(20)},
        setup::age.lte(None) => {"age", Op::Lte, Value::Null},
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.lte(true) => {"has_children", Op::Lte, Value::Bool(true)},
        setup::has_children.lte(Some(true)) => {"has_children", Op::Lte, Value::Bool(true)},
        setup::has_children.lte(None) => {"has_children", Op::Lte, Value::Null},
    );

    // for String
    assert_exprs_eq!(
        setup::username.lte("Lucy".to_string()) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        setup::username.lte(Some("Lucy".to_string())) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        setup::username.lte(None::<String>) => {"username", Op::Lte, Value::Null},
    );

    // for &str
    assert_exprs_eq!(
        setup::username.lte("Lucy") => {"username", Op::Lte, Value::String("Lucy".to_string())},
        setup::username.lte(Some("Lucy")) => {"username", Op::Lte, Value::String("Lucy".to_string())},
        setup::username.lte(None::<&str>) => {"username", Op::Lte, Value::Null},
    );
}
#[test]
pub fn test_like() {
    // for String
    assert_exprs_eq!(
        setup::username.like("Lucy".to_string()) => {"username", Op::Like, Value::String("Lucy".to_string())},
        setup::username.like(Some("Lucy".to_string())) => {"username", Op::Like, Value::String("Lucy".to_string())},
        setup::username.like(None::<String>) => {"username", Op::Like, Value::Null},
    );
    // for &str
    assert_exprs_eq!(
        setup::username.like("Lucy") => {"username", Op::Like, Value::String("Lucy".to_string())},
        setup::username.like(Some("Lucy")) => {"username", Op::Like, Value::String("Lucy".to_string())},
        setup::username.like(None::<&str>) => {"username", Op::Like, Value::Null},
    );
}
#[test]
pub fn test_not_like() {
    // for String
    assert_exprs_eq!(
        setup::username.not_like("Lucy".to_string()) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        setup::username.not_like(Some("Lucy".to_string())) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        setup::username.not_like(None::<String>) => {"username", Op::NotLike, Value::Null},
    );
    // for &str
    assert_exprs_eq!(
        setup::username.not_like("Lucy") => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        setup::username.not_like(Some("Lucy")) => {"username", Op::NotLike, Value::String("Lucy".to_string())},
        setup::username.not_like(None::<&str>) => {"username", Op::NotLike, Value::Null},
    );
}
