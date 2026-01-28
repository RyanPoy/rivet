use super::*;
use crate::ast::{Expr, Op, Value};

#[allow(non_upper_case_globals)]
mod setup {
    use crate::ast::{Operand, SelectStatement};
    use std::sync::LazyLock;

    pub const select_statement: LazyLock<SelectStatement> =
        LazyLock::new(|| SelectStatement::new().select(Operand::Column("id")));
}

#[test]
fn test_source_join_basic() {
    // 构造：users INNER JOIN orders ON users.id = orders.user_id
    let left = Source::Table { name: "users", alias: Some("u") };
    let right = Source::Table { name: "orders", alias: Some("o") };
    let on_condition = Expr::new_binary("u.id", Op::Eq, Value::I32(1)); // 简化演示

    let join_source =
        Source::Join { left: Box::new(left), right: Box::new(right), tp: JoinType::Inner, on: on_condition };

    if let Source::Join { left, right, tp, .. } = join_source {
        assert!(matches!(*left, Source::Table { name: "users", alias: Some("u") }));
        assert!(matches!(*right, Source::Table { name: "orders", alias: Some("o") }));
        assert_eq!(tp, JoinType::Inner);
    } else {
        panic!("Expected Source::Join");
    }
}

#[test]
fn test_source_nested_join() {
    // 构造更复杂的嵌套：(A JOIN B) LEFT JOIN C
    let a = Source::Table { name: "table_a", alias: None };
    let b = Source::Table { name: "table_b", alias: None };
    let c = Source::Table { name: "table_c", alias: None };
    let cond = Expr::new_binary("a.id", Op::Eq, Value::I32(1));

    let first_join = Source::Join { left: Box::new(a), right: Box::new(b), tp: JoinType::Inner, on: cond.clone() };

    let nested_join = Source::Join { left: Box::new(first_join), right: Box::new(c), tp: JoinType::Left, on: cond };

    // 验证深度
    if let Source::Join { left, tp, .. } = nested_join {
        assert_eq!(tp, JoinType::Left);
        assert!(matches!(*left, Source::Join { .. })); // 左边又是一个 Join
    } else {
        panic!("Expected nested Source::Join");
    }
}

#[test]
fn test_source_subquery_recursion() {
    // 测试循环引用：FROM (SELECT ...) AS sub
    let inner_query = setup::select_statement.clone();

    let source = Source::SubQuery { query: Box::new(inner_query), alias: "sub" };

    if let Source::SubQuery { query, alias } = source {
        assert_eq!(alias, "sub");
        assert_eq!(query.select.len(), 1);
    } else {
        panic!("Expected Source::SubQuery");
    }
}

#[test]
fn test_all_join_types_coverage() {
    let types = [JoinType::Inner, JoinType::Left, JoinType::Right, JoinType::Full, JoinType::Cross];

    for jt in types {
        let source = Source::Join {
            left: Box::new(Source::Table { name: "a", alias: None }),
            right: Box::new(Source::Table { name: "b", alias: None }),
            tp: jt,
            on: Expr::new_binary("a.id", Op::Eq, Value::I32(1)),
        };
        if let Source::Join { tp, .. } = source {
            assert_eq!(tp, jt);
        }
    }
}
