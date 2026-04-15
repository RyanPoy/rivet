use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::{ORDERS, USERS};
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{
    abs, avg, ceil, coalesce, count, count_all, floor, func, lower, max, min, sqrt, sum, upper,
};
use crate::sequel::term::param::{Param, lit};

#[test]
fn test_count_all() {
    let stmt = SelectStatement::from(&*USERS).select(count_all());
    assert_mysql!(&stmt, "SELECT COUNT(*) FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "users0""#);
}

#[test]
fn test_count_column() {
    let c = USERS.column("email");
    let stmt = SelectStatement::from(&*USERS).select(count(c));
    assert_mysql!(&stmt, "SELECT COUNT(`users0`.`email`) FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT COUNT("users0"."email") FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT COUNT("users0"."email") FROM "users" AS "users0""#);
}

#[test]
fn test_count_distinct() {
    let stmt = SelectStatement::from(&*USERS).select(count(USERS.column("city")).distinct());
    assert_mysql!(&stmt, "SELECT COUNT(DISTINCT `users0`.`city`) FROM `users` AS `users0`");
    assert_pg!(
        &stmt,
        r#"SELECT COUNT(DISTINCT "users0"."city") FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COUNT(DISTINCT "users0"."city") FROM "users" AS "users0""#
    );
}

#[test]
fn test_count_distinct_multiple() {
    let stmt = SelectStatement::from(&*USERS)
        .select(count([USERS.column("city"), USERS.column("username"), USERS.column("id")]).distinct())
        .select(USERS.column("id"));
    assert_mysql!(
        &stmt,
        "SELECT COUNT(DISTINCT `users0`.`city`, `users0`.`username`, `users0`.`id`), `users0`.`id` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COUNT(DISTINCT ("users0"."city", "users0"."username", "users0"."id")), "users0"."id" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COUNT(*), "sq0"."id" FROM (SELECT DISTINCT "users0"."city", "users0"."username", "users0"."id", "users0"."id" FROM "users" AS "users0") AS "sq0""#
    );
}

#[test]
fn test_abs_ceil_floor() {
    let stmt = SelectStatement::from(&*ORDERS)
        .select(sum(ORDERS.column("total")))
        .select(avg(ORDERS.column("price")))
        .select(max(ORDERS.column("total")))
        .select(min(ORDERS.column("price")))
        .select(abs(ORDERS.column("discount")))
        .select(ceil(ORDERS.column("price")))
        .select(floor(ORDERS.column("tax")))
        .select(lower(ORDERS.column("name")))
        .select(upper(ORDERS.column("brand_name")))
        .select(sqrt(ORDERS.column("quantity")));
    assert_mysql!(
        &stmt,
        "SELECT SUM(`orders0`.`total`), AVG(`orders0`.`price`), MAX(`orders0`.`total`), MIN(`orders0`.`price`), ABS(`orders0`.`discount`), CEIL(`orders0`.`price`), FLOOR(`orders0`.`tax`), LOWER(`orders0`.`name`), UPPER(`orders0`.`brand_name`), SQRT(`orders0`.`quantity`) FROM `orders` AS `orders0`"
    );
}

#[test]
fn test_custom_func() {
    let stmt = SelectStatement::from(&*USERS).select(func(
        "CONCAT",
        vec![USERS.column("first_name"), USERS.column("last_name")],
    ));
    assert_mysql!(
        &stmt,
        "SELECT CONCAT(`users0`.`first_name`, `users0`.`last_name`) FROM `users` AS `users0`"
    );
}

#[test]
fn test_coalesce() {
    let stmt = SelectStatement::from(&*USERS).select(coalesce(vec![
        Expr::Column(USERS.column("email")),
        lit("no-email").into(),
    ]));
    assert_mysql!(
        &stmt,
        "SELECT COALESCE(`users0`.`email`, 'no-email') FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", 'no-email') FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", 'no-email') FROM "users" AS "users0""#
    );
}

#[test]
fn test_coalesce_multiple() {
    let stmt = SelectStatement::from(&*USERS).select(coalesce(vec![
        Expr::Column(USERS.column("email")),
        Expr::Column(USERS.column("phone")),
        lit("no-contact").into(),
    ]));
    assert_mysql!(
        &stmt,
        "SELECT COALESCE(`users0`.`email`, `users0`.`phone`, 'no-contact') FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", "users0"."phone", 'no-contact') FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", "users0"."phone", 'no-contact') FROM "users" AS "users0""#
    );
}
