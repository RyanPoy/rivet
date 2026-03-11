use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::literal::Literal;
use crate::sequel::term::table::Table;
use crate::sequel::visitor::visitor;
use std::sync::LazyLock;

macro_rules! assert_mysql {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = visitor::mysql().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
    };
}

macro_rules! assert_pg {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = visitor::postgre().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
    };
}

macro_rules! assert_sqlite {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = visitor::sqlite().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
    };
}

// ============================================================================
// 基础表定义
// ============================================================================

static USERS: LazyLock<Table> = LazyLock::new(|| Table::new("users"));
static ORDERS: LazyLock<Table> = LazyLock::new(|| Table::new("orders"));
static PRODUCTS: LazyLock<Table> = LazyLock::new(|| Table::new("products"));
static CATEGORIES: LazyLock<Table> = LazyLock::new(|| Table::new("categories"));

// ============================================================================
// 1. 基础 SELECT 测试
// ============================================================================
mod select_basic {
    use super::*;
    #[test]
    fn test_select_all() {
        let stmt = SelectStatement::from(&*USERS);
        assert_mysql!(&stmt, "SELECT * FROM `users` AS `t1`", []);
        assert_pg!(&stmt, r#"SELECT * FROM "users" AS "t1""#, []);
        assert_sqlite!(&stmt, r#"SELECT * FROM "users" AS "t1""#, []);
    }

    #[test]
    fn test_select_single_column() {
        let stmt = SelectStatement::from(&*USERS).select(USERS.column("id"));
        assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1`", []);
        assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1""#, []);
        assert_sqlite!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1""#, []);
    }

    #[test]
    fn test_select_multiple_columns() {
        let stmt = SelectStatement::from(&*USERS).select(USERS.columns(["id", "name", "email"]));
        assert_mysql!(
            &stmt,
            "SELECT `t1`.`id`, `t1`.`name`, `t1`.`email` FROM `users` AS `t1`",
            []
        );
        assert_pg!(
            &stmt,
            r#"SELECT "t1"."id", "t1"."name", "t1"."email" FROM "users" AS "t1""#,
            []
        );
        assert_sqlite!(
            &stmt,
            r#"SELECT "t1"."id", "t1"."name", "t1"."email" FROM "users" AS "t1""#,
            []
        );
    }

    #[test]
    fn test_select_with_literal() {
        let stmt = SelectStatement::from(&*USERS).select([Literal::from(1), Literal::from("hello")]);
        assert_mysql!(&stmt, "SELECT 1, 'hello' FROM `users` AS `t1`", []);
        assert_pg!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "t1""#, []);
        assert_sqlite!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "t1""#, []);
    }
}

mod where_test {
    use super::*;
    #[test]
    fn test_where() {
        let id = USERS.column("id");
        let age = USERS.column("age");
        let score = USERS.column("score");
        let name = USERS.column("name");
        let country = USERS.column("country");
        let email = USERS.column("email");
        let ext = USERS.column("ext");

        let stmt = SelectStatement::from(&*USERS)
            .select(&id)
            .where_(id.eq(5))
            .where_(id.not_eq(10))
            .where_(age.gt(20))
            .where_(age.lt(100))
            .where_(score.gte(60))
            .where_(score.lte(96))
            .where_(name.like("%John%"))
            .where_(name.not_like("%Lucy%"))
            .where_(country.in_(vec!["China", "Japan"]))
            .where_(country.not_in(["USA", "England"]))
            .where_(email.not_eq(None::<i32>))
            .where_(ext.eq(None::<i32>));

        assert_mysql!(
            &stmt,
            "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`id` = ? AND `t1`.`id` <> ? AND `t1`.`age` > ? AND `t1`.`age` < ? AND `t1`.`score` >= ? AND `t1`.`score` <= ? AND `t1`.`name` LIKE ? AND `t1`.`name` NOT LIKE ? AND `t1`.`country` IN (?, ?) AND `t1`.`country` NOT IN (?, ?) AND `t1`.`email` IS NOT NULL AND `t1`.`ext` IS NULL",
            [
                5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
            ]
        );
        assert_pg!(
            &stmt,
            r#"SELECT "t1"."id" FROM "users" AS "t1" WHERE "t1"."id" = $1 AND "t1"."id" <> $2 AND "t1"."age" > $3 AND "t1"."age" < $4 AND "t1"."score" >= $5 AND "t1"."score" <= $6 AND "t1"."name" LIKE $7 AND "t1"."name" NOT LIKE $8 AND "t1"."country" IN ($9, $10) AND "t1"."country" NOT IN ($11, $12) AND "t1"."email" IS NOT NULL AND "t1"."ext" IS NULL"#,
            [
                5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
            ]
        );
        assert_sqlite!(
            &stmt,
            r#"SELECT "t1"."id" FROM "users" AS "t1" WHERE "t1"."id" = ? AND "t1"."id" <> ? AND "t1"."age" > ? AND "t1"."age" < ? AND "t1"."score" >= ? AND "t1"."score" <= ? AND "t1"."name" LIKE ? AND "t1"."name" NOT LIKE ? AND "t1"."country" IN (?, ?) AND "t1"."country" NOT IN (?, ?) AND "t1"."email" IS NOT NULL AND "t1"."ext" IS NULL"#,
            [
                5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
            ]
        );
    }

    #[test]
    fn test_where_logic() {
        let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).where_(
            USERS
                .column("active")
                .eq(true)
                .and(
                    USERS
                        .column("role")
                        .eq("admin")
                        .or(USERS.column("role").eq("superadmin")),
                )
                .and(!USERS.column("age").lt(18).or(!USERS.column("age").gt(12))),
        );
        assert_mysql!(
            &stmt,
            "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`active` = ? AND (`t1`.`role` = ? OR `t1`.`role` = ?) AND NOT (`t1`.`age` < ? OR NOT `t1`.`age` > ?)",
            [true, "admin", "superadmin", 18, 12]
        );
    }

    #[test]
    fn test_complex_precedence_auto_grouping() {
        let age_limit = USERS.column("age").lt(18).or(USERS.column("age").gt(60));
        let stmt = SelectStatement::from(&*USERS).where_(age_limit.and(USERS.column("status").eq("active")));
        assert_mysql!(
            &stmt,
            "SELECT * FROM `users` AS `t1` WHERE (`t1`.`age` < ? OR `t1`.`age` > ?) AND `t1`.`status` = ?",
            [18, 60, "active"]
        );
    }

    #[test]
    fn test_nested_not_precedence() {
        let condition = !USERS
            .column("status")
            .eq("pending")
            .or(USERS.column("status").eq("deleted"));

        let stmt = SelectStatement::from(&*USERS).where_(condition);

        // 因为 10 (OR) < 40 (NOT)，所以括号必须出现
        assert_mysql!(
            &stmt,
            "SELECT * FROM `users` AS `t1` WHERE NOT (`t1`.`status` = ? OR `t1`.`status` = ?)",
            ["pending", "deleted"]
        );
    }
}

// #[test]
// fn test_inner_join() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), ORDERS.column("total")])
//         .join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total` FROM `users` AS `t1` INNER JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id`",
//         []
//     );
// }
//
// #[test]
// fn test_left_join() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), ORDERS.column("total")])
//         .left_join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total` FROM `users` AS `t1` LEFT JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id`",
//         []
//     );
// }
//
// #[test]
// fn test_right_join() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), ORDERS.column("total")])
//         .right_join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total` FROM `users` AS `t1` RIGHT JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id`",
//         []
//     );
// }
//
// #[test]
// fn test_full_join() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), ORDERS.column("total")])
//         .full_join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total` FROM `users` AS `t1` FULL JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id`",
//         []
//     );
// }
//
// #[test]
// fn test_cross_join() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), PRODUCTS.column("name")])
//         .cross_join(&PRODUCTS);
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`name` FROM `users` AS `t1` CROSS JOIN `products` AS `t2`",
//         []
//     );
// }
//
// #[test]
// fn test_multiple_joins() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![
//             USERS.column("id"),
//             ORDERS.column("total"),
//             PRODUCTS.column("name"),
//         ])
//         .join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")))
//         .join(&PRODUCTS, ORDERS.column("product_id").eq(PRODUCTS.column("id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total`, `t3`.`name` FROM `users` AS `t1` INNER JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id` INNER JOIN `products` AS `t3` ON `t2`.`product_id` = `t3`.`id`",
//         []
//     );
// }
//
// #[test]
// fn test_mixed_join_types() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("id"), ORDERS.column("total")])
//         .left_join(&ORDERS, USERS.column("id").eq(ORDERS.column("user_id")))
//         .inner_join(&PRODUCTS, ORDERS.column("product_id").eq(PRODUCTS.column("id")));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, `t2`.`total` FROM `users` AS `t1` LEFT JOIN `orders` AS `t2` ON `t1`.`id` = `t2`.`user_id` INNER JOIN `products` AS `t3` ON `t2`.`product_id` = `t3`.`id`",
//         []
//     );
// }
//
// // ============================================================================
// // 4. DISTINCT 测试
// // ============================================================================
//
// #[test]
// fn test_distinct() {
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("city")).distinct();
//     assert_mysql!(&stmt, "SELECT DISTINCT `t1`.`city` FROM `users` AS `t1`", []);
//     assert_pg!(&stmt, r#"SELECT DISTINCT "t1"."city" FROM "users" AS "t1""#, []);
// }
//
// #[test]
// fn test_distinct_on_postgres() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("city"), USERS.column("name")])
//         .distinct_on(vec![USERS.column("city")]);
//     // PostgreSQL 支持 DISTINCT ON
//     assert_pg!(
//         &stmt,
//         r#"SELECT DISTINCT ON ("t1"."city") "t1"."city", "t1"."name" FROM "users" AS "t1""#,
//         []
//     );
//     // MySQL/SQLite 会退化为普通 DISTINCT
//     assert_mysql!(
//         &stmt,
//         "SELECT DISTINCT `t1`.`city`, `t1`.`name` FROM `users` AS `t1`",
//         []
//     );
// }
//
// // ============================================================================
// // 5. LIMIT 和 OFFSET 测试
// // ============================================================================
//
// #[test]
// fn test_limit_only() {
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(10);
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` LIMIT 10", []);
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10"#, []);
//     assert_sqlite!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10"#, []);
// }
//
// #[test]
// fn test_offset_only() {
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).offset(20);
//     // MySQL 不支持单独 OFFSET
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1`", []);
//     // PostgreSQL 和 SQLite 支持
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" OFFSET 20"#, []);
//     assert_sqlite!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" OFFSET 20"#, []);
// }
//
// #[test]
// fn test_limit_and_offset() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .limit(10)
//         .offset(20);
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` LIMIT 10 OFFSET 20", []);
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10 OFFSET 20"#, []);
//     assert_sqlite!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10 OFFSET 20"#, []);
// }
//
// // ============================================================================
// // 6. 聚合函数测试
// // ============================================================================
//
// #[test]
// fn test_count_all() {
//     let stmt = SelectStatement::from(&*USERS).select(count_all());
//     assert_mysql!(&stmt, "SELECT COUNT(*) FROM `users` AS `t1`", []);
//     assert_pg!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "t1""#, []);
//     assert_sqlite!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "t1""#, []);
// }
//
// #[test]
// fn test_count_column() {
//     let stmt = SelectStatement::from(&*USERS).select(count(USERS.column("email")));
//     assert_mysql!(&stmt, "SELECT COUNT(`t1`.`email`) FROM `users` AS `t1`", []);
//     assert_pg!(&stmt, r#"SELECT COUNT("t1"."email") FROM "users" AS "t1""#, []);
// }
//
// #[test]
// fn test_count_distinct() {
//     let stmt = SelectStatement::from(&*USERS).select(count(USERS.column("city").distinct()));
//     assert_mysql!(&stmt, "SELECT COUNT(DISTINCT `t1`.`city`) FROM `users` AS `t1`", []);
//     assert_pg!(&stmt, r#"SELECT COUNT(DISTINCT "t1"."city") FROM "users" AS "t1""#, []);
// }
//
// #[test]
// fn test_sum_avg() {
//     let stmt = SelectStatement::from(&*ORDERS).select(vec![sum(ORDERS.column("total")), avg(ORDERS.column("price"))]);
//     assert_mysql!(
//         &stmt,
//         "SELECT SUM(`t1`.`total`), AVG(`t1`.`price`) FROM `orders` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_max_min() {
//     let stmt = SelectStatement::from(&*ORDERS).select(vec![max(ORDERS.column("total")), min(ORDERS.column("price"))]);
//     assert_mysql!(
//         &stmt,
//         "SELECT MAX(`t1`.`total`), MIN(`t1`.`price`) FROM `orders` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_upper_lower() {
//     let stmt = SelectStatement::from(&*USERS).select(vec![upper(USERS.column("name")), lower(USERS.column("email"))]);
//     assert_mysql!(
//         &stmt,
//         "SELECT UPPER(`t1`.`name`), LOWER(`t1`.`email`) FROM `users` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_abs_ceil_floor() {
//     let stmt = SelectStatement::from(&*ORDERS).select(vec![
//         abs(ORDERS.column("discount")),
//         ceil(ORDERS.column("price")),
//         floor(ORDERS.column("tax")),
//     ]);
//     assert_mysql!(
//         &stmt,
//         "SELECT ABS(`t1`.`discount`), CEIL(`t1`.`price`), FLOOR(`t1`.`tax`) FROM `orders` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_sqrt() {
//     let stmt = SelectStatement::from(&*ORDERS).select(sqrt(ORDERS.column("quantity")));
//     assert_mysql!(&stmt, "SELECT SQRT(`t1`.`quantity`) FROM `orders` AS `t1`", []);
// }
//
// // ============================================================================
// // 7. 自定义函数测试
// // ============================================================================
//
// #[test]
// fn test_custom_func() {
//     let stmt = SelectStatement::from(&*USERS).select(func(
//         "CONCAT",
//         vec![USERS.column("first_name"), USERS.column("last_name")],
//     ));
//     assert_mysql!(
//         &stmt,
//         "SELECT CONCAT(`t1`.`first_name`, `t1`.`last_name`) FROM `users` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_coalesce() {
//     let stmt = SelectStatement::from(&*USERS).select(coalesce![USERS.column("email"), Literal::from("no-email")]);
//     assert_mysql!(
//         &stmt,
//         "SELECT COALESCE(`t1`.`email`, ?) FROM `users` AS `t1`",
//         ["no-email"]
//     );
// }
//
// #[test]
// fn test_coalesce_multiple() {
//     let stmt = SelectStatement::from(&*USERS).select(coalesce![
//         USERS.column("email"),
//         USERS.column("phone"),
//         Literal::from("no-contact")
//     ]);
//     assert_mysql!(
//         &stmt,
//         "SELECT COALESCE(`t1`.`email`, `t1`.`phone`, ?) FROM `users` AS `t1`",
//         ["no-contact"]
//     );
// }
//
// // ============================================================================
// // 8. 子查询测试
// // ============================================================================
//
// #[test]
// fn test_exists_subquery() {
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).where_(exists(
//         SelectStatement::from(&*ORDERS).where_(ORDERS.column("user_id").eq(USERS.column("id"))),
//     ));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `users` AS `t1` WHERE EXISTS(SELECT * FROM `orders` AS `t2` WHERE `t2`.`user_id` = `t1`.`id`)",
//         []
//     );
// }
//
// #[test]
// fn test_scalar_subquery() {
//     let subquery = SelectStatement::from(&*ORDERS)
//         .select(max(ORDERS.column("total")))
//         .where_(ORDERS.column("user_id").eq(USERS.column("id")));
//
//     let stmt = SelectStatement::from(&*USERS).select(vec![USERS.column("id"), Expr::from(subquery)]);
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id`, (SELECT MAX(`t2`.`total`) FROM `orders` AS `t2` WHERE `t2`.`user_id` = `t1`.`id`) FROM `users` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_in_subquery() {
//     let subquery = SelectStatement::from(&*ORDERS).select(ORDERS.column("user_id"));
//
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .where_(USERS.column("id").in_(vec![Expr::from(subquery)]));
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`id` FROM `users` AS `t1` WHERE `t1`.`id` IN ((SELECT `t2`.`user_id` FROM `orders` AS `t2`))",
//         []
//     );
// }
//
// // ============================================================================
// // 9. 列别名测试
// // ============================================================================
//
// #[test]
// fn test_column_alias() {
//     let stmt = SelectStatement::from(&*USERS).select(USERS.column("name").alias("username"));
//     assert_mysql!(&stmt, "SELECT `t1`.`name` AS `username` FROM `users` AS `t1`", []);
//     assert_pg!(&stmt, r#"SELECT "t1"."name" AS "username" FROM "users" AS "t1""#, []);
// }
//
// #[test]
// fn test_expression_alias() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select((USERS.column("first_name").clone() + USERS.column("last_name").clone()).alias("full_name"));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`first_name` + `t1`.`last_name` AS `full_name` FROM `users` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_func_alias() {
//     let stmt = SelectStatement::from(&*USERS).select(upper(USERS.column("name")).alias("upper_name"));
//     assert_mysql!(
//         &stmt,
//         "SELECT UPPER(`t1`.`name`) AS `upper_name` FROM `users` AS `t1`",
//         []
//     );
// }
//
// // ============================================================================
// // 10. 字面量表达式测试
// // ============================================================================
//
// #[test]
// fn test_arithmetic_operations() {
//     let stmt = SelectStatement::from(&*ORDERS).select(ORDERS.column("price") * ORDERS.column("quantity"));
//     assert_mysql!(&stmt, "SELECT `t1`.`price` * `t1`.`quantity` FROM `orders` AS `t1`", []);
// }
//
// #[test]
// fn test_addition_subtraction() {
//     let stmt = SelectStatement::from(&*ORDERS).select(vec![
//         ORDERS.column("price") + ORDERS.column("tax"),
//         ORDERS.column("price") - ORDERS.column("discount"),
//     ]);
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`price` + `t1`.`tax`, `t1`.`price` - `t1`.`discount` FROM `orders` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_division_modulo() {
//     let stmt = SelectStatement::from(&*ORDERS).select(vec![
//         ORDERS.column("total") / ORDERS.column("quantity"),
//         ORDERS.column("total") % ORDERS.column("quantity"),
//     ]);
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`total` / `t1`.`quantity`, `t1`.`total` % `t1`.`quantity` FROM `orders` AS `t1`",
//         []
//     );
// }
//
// #[test]
// fn test_mixed_arithmetic() {
//     let stmt = SelectStatement::from(&*ORDERS)
//         .select((ORDERS.column("price") * ORDERS.column("quantity") + ORDERS.column("tax")) * Literal::from(0.9));
//     assert_mysql!(
//         &stmt,
//         "SELECT `t1`.`price` * `t1`.`quantity` + `t1`.`tax` * ? FROM `orders` AS `t1`",
//         [0.9f64]
//     );
// }
//
// // ============================================================================
// // 11. 表别名测试
// // ============================================================================
//
// #[test]
// fn test_table_alias() {
//     let u = USERS.clone().alias("u");
//     let o = ORDERS.clone().alias("o");
//
//     let stmt = SelectStatement::from(&u)
//         .select(vec![u.column("id"), o.column("total")])
//         .join(&o, u.column("id").eq(o.column("user_id")));
//
//     assert_mysql!(
//         &stmt,
//         "SELECT `u`.`id`, `o`.`total` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id`",
//         []
//     );
// }
//
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
//
// // ============================================================================
// // 16. 多数据库方言差异测试
// // ============================================================================
//
// #[test]
// fn test_dialect_differences() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(USERS.column("id"))
//         .limit(10)
//         .offset(5);
//
//     // 所有数据库都支持 LIMIT + OFFSET
//     assert_mysql!(&stmt, "SELECT `t1`.`id` FROM `users` AS `t1` LIMIT 10 OFFSET 5", []);
//     assert_pg!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10 OFFSET 5"#, []);
//     assert_sqlite!(&stmt, r#"SELECT "t1"."id" FROM "users" AS "t1" LIMIT 10 OFFSET 5"#, []);
// }
//
// #[test]
// fn test_postgresql_distinct_on() {
//     let stmt = SelectStatement::from(&*USERS)
//         .select(vec![USERS.column("city"), USERS.column("name")])
//         .distinct_on(vec![USERS.column("city")]);
//
//     // PostgreSQL 支持 DISTINCT ON
//     assert_pg!(
//         &stmt,
//         r#"SELECT DISTINCT ON ("t1"."city") "t1"."city", "t1"."name" FROM "users" AS "t1""#,
//         []
//     );
//
//     // MySQL/SQLite 会退化为普通 DISTINCT
//     assert_mysql!(
//         &stmt,
//         "SELECT DISTINCT `t1`.`city`, `t1`.`name` FROM `users` AS `t1`",
//         []
//     );
//     assert_sqlite!(
//         &stmt,
//         r#"SELECT DISTINCT "t1"."city", "t1"."name" FROM "users" AS "t1""#,
//         []
//     );
// }
