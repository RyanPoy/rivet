use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::{ORDERS, USERS};
use crate::sequel::term::comparable::Comparable;
use crate::sequel::term::func::upper;

#[test]
fn test_column_alias() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("name").alias("username"));
    assert_mysql!(&stmt, "SELECT `users0`.`name` AS `username` FROM `users` AS `users0`");
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#
    );
}

#[test]
fn test_expression_alias() {
    let stmt = SelectStatement::from(&*USERS)
        .select((USERS.column("first_name") + USERS.column("last_name").clone()).alias("full_name"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`first_name` + `users0`.`last_name` AS `full_name` FROM `users` AS `users0`"
    );
}

#[test]
fn test_func_alias() {
    let stmt = SelectStatement::from(&*USERS).select(upper(USERS.column("name")).alias("upper_name"));
    assert_mysql!(
        &stmt,
        "SELECT UPPER(`users0`.`name`) AS `upper_name` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#
    );
}

#[test]
fn test_table_alias() {
    let u = USERS.clone().alias("u");
    let o = ORDERS.clone().alias("o");

    let stmt = SelectStatement::from(&u)
        .select(vec![u.column("id"), o.column("total")])
        .join(&o, u.column("id").eq(o.column("user_id")));

    assert_mysql!(
        &stmt,
        "SELECT `u`.`id`, `o`.`total` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id`"
    );
}
