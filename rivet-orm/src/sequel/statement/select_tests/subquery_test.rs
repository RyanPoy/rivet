use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;
use crate::sequel::term::func::{exists, max};

#[test]
fn test_exists_subquery() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).where_(
        exists(SelectStatement::from((*ORDERS).clone()).where_(ORDERS.column("user_id").eq(USERS.column("id")))).into(),
    );
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE EXISTS(SELECT * FROM `orders` AS `orders0` WHERE `orders0`.`user_id` = `users0`.`id`)",
        []
    );
}

#[test]
fn test_scalar_subquery() {
    let subquery = SelectStatement::from(&*ORDERS)
        .select(max(ORDERS.column("total")))
        .where_(ORDERS.column("user_id").eq(USERS.column("id")));

    let stmt = SelectStatement::from(&*USERS).select(vec![USERS.column("id"), Expr::from(subquery)]);

    assert_mysql!(
        &stmt,
        "SELECT `t1`.`id`, (SELECT MAX(`t2`.`total`) FROM `orders` AS `t2` WHERE `t2`.`user_id` = `t1`.`id`) FROM `users` AS `t1`",
        []
    );
}

#[test]
fn test_in_subquery() {
    let subquery = SelectStatement::from(&*ORDERS).select(ORDERS.column("user_id"));

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("id").in_(vec![Expr::from(subquery)]));

    assert_mysql!(
        &stmt,
        "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`id` IN ((SELECT `t2`.`user_id` FROM `orders` AS `t2`))",
        []
    );
}
