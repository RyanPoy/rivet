use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::ORDERS;
use crate::sequel::term::param::{Param, lit};

#[test]
fn test_arithmetic_operations() {
    let stmt = SelectStatement::from(&*ORDERS).select(ORDERS.column("price") * ORDERS.column("quantity"));
    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`price` * `orders0`.`quantity` FROM `orders` AS `orders0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "orders0"."price" * "orders0"."quantity" FROM "orders" AS "orders0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "orders0"."price" * "orders0"."quantity" FROM "orders" AS "orders0""#
    );
}

#[test]
fn test_addition_subtraction() {
    let stmt = SelectStatement::from(&*ORDERS).select(vec![
        ORDERS.column("price") + ORDERS.column("tax"),
        ORDERS.column("price") - ORDERS.column("discount"),
    ]);
    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`price` + `orders0`.`tax`, `orders0`.`price` - `orders0`.`discount` FROM `orders` AS `orders0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "orders0"."price" + "orders0"."tax", "orders0"."price" - "orders0"."discount" FROM "orders" AS "orders0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "orders0"."price" + "orders0"."tax", "orders0"."price" - "orders0"."discount" FROM "orders" AS "orders0""#
    );
}

#[test]
fn test_division_modulo() {
    let stmt = SelectStatement::from(&*ORDERS).select(vec![
        ORDERS.column("total") / ORDERS.column("quantity"),
        ORDERS.column("total") % ORDERS.column("quantity"),
    ]);
    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`total` / `orders0`.`quantity`, `orders0`.`total` % `orders0`.`quantity` FROM `orders` AS `orders0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "orders0"."total" / "orders0"."quantity", "orders0"."total" % "orders0"."quantity" FROM "orders" AS "orders0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "orders0"."total" / "orders0"."quantity", "orders0"."total" % "orders0"."quantity" FROM "orders" AS "orders0""#
    );
}

#[test]
fn test_mixed_arithmetic() {
    let stmt = SelectStatement::from(&*ORDERS)
        .select((ORDERS.column("price") * ORDERS.column("quantity") + ORDERS.column("tax")) * lit(0.9));
    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`price` * `orders0`.`quantity` + `orders0`.`tax` * 0.9 FROM `orders` AS `orders0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "orders0"."price" * "orders0"."quantity" + "orders0"."tax" * 0.9 FROM "orders" AS "orders0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "orders0"."price" * "orders0"."quantity" + "orders0"."tax" * 0.9 FROM "orders" AS "orders0""#
    );
}
