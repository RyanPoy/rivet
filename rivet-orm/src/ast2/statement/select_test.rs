use crate::ast2::sql::visitor::Visitor;
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::table_ref::TableRef;

#[test]
fn test_select_empty_from_single() {
    let stmt = SelectStatement::new().from("users");
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users""#.to_string());

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users""#.to_string());
}
#[test]
fn test_select_empty_from_single_with_alias() {
    let t = TableRef::from("users").alias("u");

    let stmt = SelectStatement::new().from(t);
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users` AS `u`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users" AS "u""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users" AS "u""#);
}
#[test]
fn test_select_empty_from_multiple() {
    let stmt = SelectStatement::new().from("users").from("orders").from("products");
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);
}

#[test]
fn test_select_empty_from_multiple_with_alias() {
    let t1 = TableRef::from("users").alias("u");
    let t2 = TableRef::from("orders").alias("o");
    let t3 = TableRef::from("products").alias("p");

    let stmt = SelectStatement::new().from(t1).from(t2).from(t3);
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users` AS `u`, `orders` AS `o`, `products` AS `p`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users" AS "u", "orders" AS "o", "products" AS "p""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users" AS "u", "orders" AS "o", "products" AS "p""#);
}

#[test]
fn test_select_empty_from_multiple_2() {
    let stmt = SelectStatement::new().from_many(vec!["users", "orders", "products"]);
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);
}

// #[test]
// fn test_select_columns_from_table_mysql() {
//     let stmt = SelectStatement::new();
//     stmt.select_clause = vec!["id".to_string(), "name".to_string(), "email".to_string()];
//     stmt = stmt.from("users");
//
// let mut v = Visitor::mysql();
//     let render = v;
//     let sql = stmt.render_by(&render);
//
//     assert_eq!(sql, "SELECT id, name, email FROM `users`");
// }
//
// // 测试 FROM 子句为空的情况
// #[test]
// fn test_select_without_from() {
//     let stmt = SelectStatement::new();
//
// let mut v = Visitor::mysql();
//     let render = v;
//     let sql = stmt.render_by(&render);
//
//     assert_eq!(sql, "SELECT *");
// }
//
// // 测试 From trait 的实现
// #[test]
// fn test_from_trait_for_table_ref() {
//     let from_str: TableRef = "users".into();
//     let from_string: TableRef = "users".to_string().into();
//     let from_named_table: TableRef = NamedTable::new("users").into();
//
// let mut v = Visitor::mysql();
//     let render = v;
//
//     assert_eq!(from_str.render_by(&render), "`users`");
//     assert_eq!(from_string.render_by(&render), "`users`");
//     assert_eq!(from_named_table.render_by(&render), "`users`");
// }
//
// // 测试链式调用
// #[test]
// fn test_chained_method_calls() {
//     let stmt = SelectStatement::new()
//         .from(NamedTable::new("users"))
//         .from("orders");
//
// let mut v = Visitor::mysql();
//     let render = v;
//     let sql = stmt.render_by(&render);
//
//     assert_eq!(sql, "SELECT * FROM `users`, `orders`");
// }
