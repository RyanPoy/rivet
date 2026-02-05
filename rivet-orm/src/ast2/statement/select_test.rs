use super::*;
use crate::ast2::render::SqlRender;
use crate::ast2::statement::select::SelectStatement;

#[test]
fn test_select_empty_from_single() {
    let mut stmt = SelectStatement::new().from("users");
    let sql = stmt.render_by(&mut SqlRender::mysql());
    assert_eq!(sql, "SELECT * FROM `users`".to_string());

    let sql = stmt.render_by(&mut SqlRender::postgre());
    assert_eq!(sql, r#"SELECT * FROM "users""#.to_string());

    let sql = stmt.render_by(&mut SqlRender::sqlite());
    assert_eq!(sql, r#"SELECT * FROM "users""#.to_string());
}

#[test]
fn test_select_empty_from_multiple() {
    let stmt = SelectStatement::new().from("users").from("orders").from("products");
    let sql = stmt.render_by(&mut SqlRender::mysql());
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`".to_string());

    let sql = stmt.render_by(&mut SqlRender::postgre());
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#.to_string());

    let sql = stmt.render_by(&mut SqlRender::sqlite());
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#.to_string());
}

#[test]
fn test_select_empty_from_multiple_2() {
    let stmt = SelectStatement::new().from_many(vec!["users", "orders", "products"]);
    let sql = stmt.render_by(&mut SqlRender::mysql());
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`".to_string());

    let sql = stmt.render_by(&mut SqlRender::postgre());
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#.to_string());

    let sql = stmt.render_by(&mut SqlRender::sqlite());
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#.to_string());
}

// #[test]
// fn test_select_columns_from_table_mysql() {
//     let mut stmt = SelectStatement::new();
//     stmt.select_clause = vec!["id".to_string(), "name".to_string(), "email".to_string()];
//     stmt = stmt.from("users");
//
//     let render = SqlRender::mysql();
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
//     let render = SqlRender::mysql();
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
//     let render = SqlRender::mysql();
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
//     let render = SqlRender::mysql();
//     let sql = stmt.render_by(&render);
//
//     assert_eq!(sql, "SELECT * FROM `users`, `orders`");
// }
