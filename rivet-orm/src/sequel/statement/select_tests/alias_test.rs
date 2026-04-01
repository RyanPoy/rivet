use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;
use crate::sequel::term::func::upper;


#[test]
fn test_column_alias() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("name").alias("username"));
    assert_mysql!(&stmt, "SELECT `users0`.`name` AS `username` FROM `users` AS `users0`", []);
    assert_pg!(&stmt, r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#, []);
    assert_sqlite!(&stmt, r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#, []);
}

// #[test]
// fn test_expression_alias() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select((USERS.column("first_name").clone() + USERS.column("last_name").clone()).alias("full_name"));
//     assert_mysql!(
//         &stmt,
//         "SELECT `users0`.`first_name` + `users0`.`last_name` AS `full_name` FROM `users` AS `users0`",
//         []
//     );
// }

#[test]
fn test_func_alias() {
    let stmt = SelectStatement::from(&*USERS).select(upper(USERS.column("name")).alias("upper_name"));
    assert_mysql!( &stmt, "SELECT UPPER(`users0`.`name`) AS `upper_name` FROM `users` AS `users0`", [] );
    assert_pg!( &stmt, r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#, [] );
    assert_sqlite!( &stmt, r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#, [] );
}
