// // ============================================================================
// // 12. 锁定子句测试 (FOR UPDATE 等)
// // ============================================================================
//
// #[test]
// fn test_for_update() {
//     use crate::sequel::term::lock::{Lock, Wait};
//
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .for_update(Lock::Update, Wait::NoWait);
//
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` FOR UPDATE NOWAIT", []);
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" FOR UPDATE NOWAIT"#, []);
// }
//
// #[test]
// fn test_for_share() {
//     use crate::sequel::term::lock::{Lock, Wait};
//
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .for_update(Lock::Share, Wait::SkipLocked);
//
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` FOR SHARE SKIP LOCKED", []);
// }
//
// // ============================================================================
// // 13. 索引提示测试 (MySQL 特有)
// // ============================================================================
//
// #[test]
// fn test_force_index() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .force_index("idx_users_email");
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `users` AS `t1` FORCE INDEX (idx_users_email)",
//         []
//     );
// }
//
// // ============================================================================
// // 14. 复杂组合测试
// // ============================================================================
//
// #[test]
// fn test_complex_query() {
//     let u = USERS.clone().alias("u");
//     let o = ORDERS.clone().alias("o");
//     let p = PRODUCTS.clone().alias("p");
//
//     let stmt = SelectStatement::from(&u)
//         .select(vec![
//             u.column("id"),
//             u.column("name"),
//             sum(o.column("total")).alias("total_spent"),
//             count(o.column("id")).alias("order_count"),
//         ])
//         .join(&o, u.column("id").eq(o.column("user_id")))
//         .join(&p, o.column("product_id").eq(p.column("id")))
//         .where_(o.column("total").gt(100).and(p.column("category_id").eq(5)))
//         .distinct()
//         .limit(10)
//         .offset(20);
//
//     assert_mysql!(
//         &stmt,
//         "SELECT DISTINCT `u`.`id`, `u`.`name`, SUM(`o`.`total`) AS `total_spent`, COUNT(`o`.`id`) AS `order_count` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id` INNER JOIN `products` AS `p` ON `o`.`product_id` = `p`.`id` WHERE `o`.`total` > ? AND `p`.`category_id` = ? LIMIT 10 OFFSET 20",
//         [100i64, 5i64]
//     );
// }
//
// #[test]
// fn test_nested_subquery_with_join() {
//     let subquery = SelectStatement::from(&*ORDERS)
//         .select(max(ORDERS.column("total")))
//         .where_(ORDERS.column("status").eq("completed"));
//
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![
//             USERS.column("id"),
//             Expr::from(subquery.clone()).alias("max_order"),
//         ])
//         .where_(Expr::from(subquery).gt(1000));
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, (SELECT MAX(`t2`.`total`) FROM `orders` AS `t2` WHERE `t2`.`status` = ?) AS `max_order` FROM `users` AS `t1` WHERE (SELECT MAX(`t3`.`total`) FROM `orders` AS `t3` WHERE `t3`.`status` = ?) > ?",
//         ["completed", "completed", 1000i64]
//     );
// }
//
// // ============================================================================
// // 15. 边界情况测试
// // ============================================================================
//
// #[test]
// fn test_empty_select_becomes_star() {
//     // 当没有指定 select 列时，应该生成 SELECT *
//     let stmt = SelectStatement::from(&*USERS);
//     assert_mysql!(&stmt, "SELECT * FROM `users` AS `t1`", []);
// }
//
// #[test]
// fn test_zero_limit() {
//     // limit(0) 应该被忽略
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(0);
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1`", []);
// }
//
// #[test]
// fn test_string_with_quotes() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("name"))
//         .where_(USERS.column("name").eq("O'Brien"));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`name` FROM `users` AS `t1` WHERE `t1`.`name` = ?",
//         ["O'Brien"]
//     );
// }
//
// #[test]
// fn test_string_with_backslash() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("path"))
//         .where_(USERS.column("path").eq("C:\\Users\\test"));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`path` FROM `users` AS `t1` WHERE `t1`.`path` = ?",
//         ["C:\\Users\\test"]
//     );
// }
//
// #[test]
// fn test_boolean_literals() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .where_(USERS.column("active").eq(true));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`active` = TRUE",
//         [true]
//     );
//
//     let stmt2 = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .where_(USERS.column("active").eq(false));
//     assert_mysql!(
//         &stmt2,
//         "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`active` = FALSE",
//         [false]
//     );
// }
//
// #[test]
// fn test_float_literals() {
//     let stmt = SelectStatement::from(&*ORDERS)
//         .select(ORDERS.column("price"))
//         .where_(ORDERS.column("price").eq(19.99));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`price` FROM `orders` AS `t1` WHERE `t1`.`price` = ?",
//         [19.99f64]
//     );
// }
//
// #[test]
// fn test_null_literal() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .where_(USERS.column("deleted_at").eq(Literal::Null));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`deleted_at` IS NULL",
//         []
//     );
// }
//
// #[test]
// fn test_date_time_literals() {
//     use crate::sequel::term::calendar::{Date, DateTime, Time};
//
//     let date = Date::new(2024, 1, 15);
//     let time = Time::new(10, 30, 0);
//     let datetime = DateTime::new(date, time);
//
//     let stmt = SelectStatement::from(&*ORDERS)
//         .select(ORDERS.column("id"))
//         .where_(ORDERS.column("created_at").eq(datetime));
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `orders` AS `t1` WHERE `t1`.`created_at` = ?",
//         [datetime]
//     );
// }
