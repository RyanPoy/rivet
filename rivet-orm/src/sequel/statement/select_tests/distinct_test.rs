use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::upper;

#[test]
fn test_distinct() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("city")).distinct();
    assert_mysql!(&stmt, "SELECT DISTINCT `users0`.`city` FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT DISTINCT "users0"."city" FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT DISTINCT "users0"."city" FROM "users" AS "users0""#);
}

#[test]
fn test_distinct_on() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("city"))
        .select(USERS.column("name"))
        .distinct_on(vec![
            Expr::from(upper(USERS.column("city"))),
            USERS.column("age").into(),
        ]);

    assert_mysql!(
        &stmt,
        "SELECT DISTINCT `users0`.`city`, `users0`.`name` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT DISTINCT ON (UPPER("users0"."city"), "users0"."age") "users0"."city", "users0"."name" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT DISTINCT "users0"."city", "users0"."name" FROM "users" AS "users0""#
    );
}
