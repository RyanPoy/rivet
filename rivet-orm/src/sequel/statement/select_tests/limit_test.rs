use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;

#[test]
fn test_limit_0() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(0);
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
}

#[test]
fn test_limit_only() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(10);
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0` LIMIT 10");
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10"#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10"#);
}

#[test]
fn test_offset_only() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).offset(20);
    // MySQL 不支持单独 OFFSET
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
    // PostgreSQL 和 SQLite 支持
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" OFFSET 20"#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" OFFSET 20"#);
}

#[test]
fn test_limit_and_offset() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .limit(10)
        .offset(20);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` LIMIT 10 OFFSET 20"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10 OFFSET 20"#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10 OFFSET 20"#
    );
}
