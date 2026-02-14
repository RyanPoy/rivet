use crate::ast2::sql::visitor::Visitor;
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::table_ref::TableRef;

#[test]
fn test_select_without_from() {
    let stmt = SelectStatement::new();

    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT *");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT *");

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT *");
}

#[test]
fn test_select_empty_from_single() {
    let stmt = SelectStatement::new().from("users"); // from(&str)
    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT * FROM `users`");

    let stmt = SelectStatement::new().from(NamedTable { name: "users".to_string(), alias: None }); // from(NamedTable)
    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT * FROM "users""#.to_string());

    let stmt = SelectStatement::new().from(TableRef::NamedTable(NamedTable { name: "users".to_string(), alias: None })); // from(TableRef)
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
    let stmt1 = SelectStatement::new().from("users").from("orders").from("products");
    let stmt2 = SelectStatement::new().from_many(vec!["users", "orders", "products"]);

    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt1).finish();
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`");

    let sql = v.reset().visit_select_statement(&stmt2).finish();
    assert_eq!(sql, "SELECT * FROM `users`, `orders`, `products`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt1).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);

    let sql = v.reset().visit_select_statement(&stmt2).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt1).finish();
    assert_eq!(sql, r#"SELECT * FROM "users", "orders", "products""#);

    let sql = v.reset().visit_select_statement(&stmt2).finish();
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
fn test_select_columns_from_table_mysql() {
    let stmt = SelectStatement::new().select_many(vec!["id", "name", "email"]).from("users");

    let mut v = Visitor::mysql();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, "SELECT `id`, `name`, `email` FROM `users`");

    let mut v = Visitor::postgre();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT "id", "name", "email" FROM "users""#);

    let mut v = Visitor::sqlite();
    let sql = v.visit_select_statement(&stmt).finish();
    assert_eq!(sql, r#"SELECT "id", "name", "email" FROM "users""#);
}
