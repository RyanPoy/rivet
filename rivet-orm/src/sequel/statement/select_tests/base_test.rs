use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;
use crate::sequel::term::param::{Param, lit};

#[test]
fn test_select_all() {
    let stmt = SelectStatement::from(&*USERS);
    assert_mysql!(&stmt, "SELECT * FROM `users` AS `users0`", []);
    assert_pg!(&stmt, r#"SELECT * FROM "users" AS "users0""#, []);
    assert_sqlite!(&stmt, r#"SELECT * FROM "users" AS "users0""#, []);
}

#[test]
fn test_select_single_column() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id"));
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`", []);
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#, []);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#, []);
}

#[test]
fn test_select_multiple_columns() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(USERS.column("name"))
        .select(USERS.column("email"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users0`.`name`, `users0`.`email` FROM `users` AS `users0`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#,
        []
    );
}

#[test]
fn test_select_with_literal() {
    let stmt = SelectStatement::from(&*USERS).select(lit(1)).select(lit("hello"));
    assert_mysql!(&stmt, "SELECT 1, 'hello' FROM `users` AS `users0`", []);
    assert_pg!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#, []);
    assert_sqlite!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#, []);
}
