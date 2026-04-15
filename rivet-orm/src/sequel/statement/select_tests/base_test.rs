use crate::prelude::*;
use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::{ORDERS, PRODUCTS, USERS};
use crate::sequel::term::func::{count, sum};
use crate::sequel::term::param::lit;

#[test]
fn test_select_all() {
    let stmt = SelectStatement::from(&*USERS);
    assert_mysql!(&stmt, "SELECT * FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT * FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT * FROM "users" AS "users0""#);
}

#[test]
fn test_select_single_column() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id"));
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
}

#[test]
fn test_select_multiple_columns() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(USERS.column("name"))
        .select(USERS.column("email"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users0`.`name`, `users0`.`email` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#
    );
}

#[test]
fn test_select_with_literal() {
    let stmt = SelectStatement::from(&*USERS).select(lit(1)).select(lit("hello"));
    assert_mysql!(&stmt, "SELECT 1, 'hello' FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#);
}

#[test]
fn test_complex_query() {
    let u = USERS.clone().alias("u");
    let o = ORDERS.clone().alias("o");
    let p = PRODUCTS.clone().alias("p");

    let stmt = SelectStatement::from(&u)
        .select(vec![
            u.column("id").into(),
            u.column("name").into(),
            sum(o.column("total")).alias("total_spent"),
            count(o.column("id")).alias("order_count"),
        ])
        .join(&o, u.column("id").eq(o.column("user_id")))
        .join(&p, o.column("product_id").eq(p.column("id")))
        .where_(o.column("total").gt(100).and(p.column("category_id").eq(5)))
        .distinct()
        .limit(10)
        .offset(20);

    assert_mysql!(
        &stmt,
        "SELECT DISTINCT `u`.`id`, `u`.`name`, SUM(`o`.`total`) AS `total_spent`, COUNT(`o`.`id`) AS `order_count` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id` INNER JOIN `products` AS `p` ON `o`.`product_id` = `p`.`id` WHERE `o`.`total` > ? AND `p`.`category_id` = ? LIMIT 10 OFFSET 20",
        [100i64, 5i64]
    );
}
