use super::*;
use crate::schema::col::Col;
use crate::sequel::ast::Value;
use rivet_orm_macros::table;

#[allow(non_upper_case_globals)]
mod setup {
    use crate::orm::Col;
    pub const age: Col<i32> = Col::new("age");
    pub const has_children: Col<bool> = Col::new("has_children");
    pub const username: Col<String> = Col::new("username");
}

#[test]
fn test_col_define() {
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
    }

    let user = User {
        id: 1,
        name: "Luly".to_string(),
        age: 30,
        nick_name: None,
        gender: "male",
        checked: true,
        temp: "fuck".to_string(),
    };
    assert_eq!(User::id, Col::<i32>::new("id"));
    assert_eq!(User::name, Col::<String>::new("username"));
    assert_eq!(User::age, Col::<u32>::new("age"));
    assert_eq!(User::nick_name, Col::<String>::new("nick_name"));
    assert_eq!(User::checked, Col::<bool>::new("checked"));
    assert_eq!(User::gender, Col::<String>::new("gender"));
    assert_eq!(user.id, 1);
}

macro_rules! assert_exprs_eq {
    ($($expr:expr => {$left:expr, $op:path, $right:expr}),* $(,)?) => {
        $(
            assert_eq!(
                $expr,
                Expr::Binary { left: Operand::Column{name: $left, alias: None}, op: $op, right: Operand::Value($right), }
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

#[test]
pub fn test_in() {
    // for number
    assert_exprs_eq!(
        setup::age.in_([20, 30]) => { "age", Op::In, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.in_(vec![20, 30]) => { "age", Op::In, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.in_([Some(20), Some(30)]) => { "age", Op::In, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.in_(vec![Some(20), Some(30)]) => { "age", Op::In, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.in_([None::<i32>, None::<i32>]) => { "age", Op::In, Value::List(vec![Value::Null, Value::Null]) },
        setup::age.in_(vec![None::<i32>, None::<i32>]) => { "age", Op::In, Value::List(vec![Value::Null, Value::Null]) },
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.in_([true, false]) => { "has_children", Op::In, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.in_(vec![true, false]) => { "has_children", Op::In, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.in_([Some(true), Some(false)]) => { "has_children", Op::In, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.in_(vec![Some(true), Some(false)]) => { "has_children", Op::In, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.in_([None::<bool>, None::<bool>]) => { "has_children", Op::In, Value::List(vec![Value::Null, Value::Null]) },
        setup::has_children.in_(vec![None::<bool>, None::<bool>]) => { "has_children", Op::In, Value::List(vec![Value::Null, Value::Null]) },
    );

    // for String
    assert_exprs_eq!(
        setup::username.in_(["Lucy", "Bob"]) => { "username", Op::In, Value::List(vec![Value::String("Lucy".to_string()),Value::String("Bob".to_string())])},
        setup::username.in_(vec!["Lucy", "Bob"]) => { "username", Op::In, Value::List(vec![ Value::String("Lucy".to_string()), Value::String("Bob".to_string()) ]) },
        setup::username.in_([Some("Lucy"), Some("Bob")]) => { "username", Op::In, Value::List(vec![Value::String("Lucy".to_string()),Value::String("Bob".to_string())])},
        setup::username.in_(vec![Some("Lucy"), Some("Bob")]) => { "username", Op::In, Value::List(vec![ Value::String("Lucy".to_string()), Value::String("Bob".to_string()) ]) },
        setup::username.in_([None::<&str>, None::<&str>]) => { "username", Op::In, Value::List(vec![Value::Null, Value::Null])},
        setup::username.in_(vec![None::<&str>, None::<&str>]) => { "username", Op::In, Value::List(vec![ Value::Null, Value::Null]) },
    );
}

#[test]
pub fn test_not_in() {
    // for number
    assert_exprs_eq!(
        setup::age.not_in([20, 30]) => { "age", Op::NotIn, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.not_in(vec![20, 30]) => { "age", Op::NotIn, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.not_in([Some(20), Some(30)]) => { "age", Op::NotIn, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.not_in(vec![Some(20), Some(30)]) => { "age", Op::NotIn, Value::List(vec![Value::I32(20), Value::I32(30)]) },
        setup::age.not_in([None::<i32>, None::<i32>]) => { "age", Op::NotIn, Value::List(vec![Value::Null, Value::Null]) },
        setup::age.not_in(vec![None::<i32>, None::<i32>]) => { "age", Op::NotIn, Value::List(vec![Value::Null, Value::Null]) },
    );

    // for bool
    assert_exprs_eq!(
        setup::has_children.not_in([true, false]) => { "has_children", Op::NotIn, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.not_in(vec![true, false]) => { "has_children", Op::NotIn, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.not_in([Some(true), Some(false)]) => { "has_children", Op::NotIn, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.not_in(vec![Some(true), Some(false)]) => { "has_children", Op::NotIn, Value::List(vec![Value::Bool(true), Value::Bool(false)]) },
        setup::has_children.not_in([None::<bool>, None::<bool>]) => { "has_children", Op::NotIn, Value::List(vec![Value::Null, Value::Null]) },
        setup::has_children.not_in(vec![None::<bool>, None::<bool>]) => { "has_children", Op::NotIn, Value::List(vec![Value::Null, Value::Null]) },
    );

    // for String
    assert_exprs_eq!(
        setup::username.not_in(["Lucy", "Bob"]) => { "username", Op::NotIn, Value::List(vec![Value::String("Lucy".to_string()),Value::String("Bob".to_string())])},
        setup::username.not_in(vec!["Lucy", "Bob"]) => { "username", Op::NotIn, Value::List(vec![ Value::String("Lucy".to_string()), Value::String("Bob".to_string()) ]) },
        setup::username.not_in([Some("Lucy"), Some("Bob")]) => { "username", Op::NotIn, Value::List(vec![Value::String("Lucy".to_string()),Value::String("Bob".to_string())])},
        setup::username.not_in(vec![Some("Lucy"), Some("Bob")]) => { "username", Op::NotIn, Value::List(vec![ Value::String("Lucy".to_string()), Value::String("Bob".to_string()) ]) },
        setup::username.not_in([None::<&str>, None::<&str>]) => { "username", Op::NotIn, Value::List(vec![Value::Null, Value::Null])},
        setup::username.not_in(vec![None::<&str>, None::<&str>]) => { "username", Op::NotIn, Value::List(vec![ Value::Null, Value::Null]) },
    );
}

#[test]
fn test_logical_operators_chaining() {
    let age = setup::age;
    let name = setup::username;

    // 1. 测试简单的 AND 组合，构建: age > 18 AND name = "Lucy"
    let expr_and = age.gt(18).and(name.eq("Lucy"));

    assert_eq!(expr_and, Expr::And { left: Box::new(age.gt(18)), right: Box::new(name.eq("Lucy")) });

    // 2. 测试 OR 与嵌套，构建: (age > 18 AND name = "Lucy") OR age < 10
    let expr_or = expr_and.or(age.lt(10));
    if let Expr::Or { left, right } = expr_or {
        assert!(matches!(*left, Expr::And { .. }));
        assert_eq!(*right, age.lt(10));
    } else {
        panic!("Root should be OR");
    }

    // 3. 测试 NOT 运算符，构建: NOT (name = "Lucy")
    let expr_not = name.eq("Lucy").not();
    assert_eq!(expr_not, Expr::Not { expr: Box::new(name.eq("Lucy")) });
}

#[test]
fn test_complex_mixed_logic() {
    let age = setup::age;
    let name = setup::username;
    let has_child = setup::has_children;

    // 构造一个复杂的业务逻辑场景：
    // (年龄在 20~30 之间 且 名字是 Lucy) 或者 (没有孩子 且 名字不是 Bob)
    let complex = (age.gte(20).and(age.lte(30)).and(name.eq("Lucy"))).or(has_child.eq(false).and(name.ne("Bob")));

    // 这种测试主要验证链式调用的返回值依然是 Expr，且可以无限嵌套
    match complex {
        Expr::Or { left, right } => {
            assert!(matches!(*left, Expr::And { .. }));
            assert!(matches!(*right, Expr::And { .. }));
        }
        _ => panic!("Complex expression structure mismatch"),
    }
}
