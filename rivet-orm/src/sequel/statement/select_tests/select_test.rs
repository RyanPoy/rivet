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
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` FOR UPDATE NOWAIT");
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" FOR UPDATE NOWAIT"#);
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
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` FOR SHARE SKIP LOCKED");
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
//         "SELECT `t1`.`id` FROM `users` AS `t1` FORCE INDEX (idx_users_email)"
//     );
// }
//

//
//
// #[test]
// #[ignore = "rivet-orm 可能不支持 order_by() 方法"]
// fn test_methods_generative_order_by() {}
//
// #[test]
// #[ignore = "rivet-orm 可能不支持 group_by() 方法"]
// fn test_methods_generative_group_by() {}
