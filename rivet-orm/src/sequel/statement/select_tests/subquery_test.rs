use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::{ORDERS, USERS};
use crate::sequel::term::comparable::Comparable;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{exists, max};

#[test]
fn test_exists_subquery() {
    let where_clause = ORDERS.column("user_id").eq(USERS.column("id"));
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(exists(SelectStatement::from(&*ORDERS).where_(where_clause)));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE EXISTS((SELECT * FROM `orders` AS `orders0` WHERE `orders0`.`user_id` = `users0`.`id`))",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS((SELECT * FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id"))"#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS((SELECT * FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id"))"#,
        []
    );
}

#[test]
fn test_scalar_subquery() {
    let subquery = SelectStatement::from(&*ORDERS)
        .select(max(ORDERS.column("total")))
        .where_(ORDERS.column("user_id").eq(USERS.column("id")));

    let stmt = SelectStatement::from(&*USERS).select(vec![USERS.column("id").into(), Expr::from(subquery)]);

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, (SELECT MAX(`orders0`.`total`) FROM `orders` AS `orders0` WHERE `orders0`.`user_id` = `users0`.`id`) FROM `users` AS `users0`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", (SELECT MAX("orders0"."total") FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id") FROM "users" AS "users0""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", (SELECT MAX("orders0"."total") FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id") FROM "users" AS "users0""#,
        []
    );
}

#[test]
fn test_in_subquery() {
    let subquery = SelectStatement::from(&*ORDERS).select(ORDERS.column("user_id"));

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("id").in_(vec![subquery]));

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`id` IN ((SELECT `orders0`.`user_id` FROM `orders` AS `orders0`))",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" IN ((SELECT "orders0"."user_id" FROM "orders" AS "orders0"))"#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" IN ((SELECT "orders0"."user_id" FROM "orders" AS "orders0"))"#,
        []
    );
}
