use crate::prelude::*;
use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{
    abs, avg, ceil, coalesce, count, count_all, exists, floor, func, lower, max, min, sqrt, sum, upper,
};

use crate::model::model::Model;
use crate::sequel::term::calendar::{Date, DateTime};
use crate::sequel::term::table::Table;
use std::sync::LazyLock;

macro_rules! assert_dialect {
    ($visitor:expr, $stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {{
        let (sql, params_relt) = $visitor.visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::param::ParamData> = vec![$($params.into()),*]
            .into_iter()
            .map(|p: crate::sequel::term::param::Param| p.data().unwrap().clone())
            .collect();
        assert_eq!(params_relt, expected);
    }};
    ($visitor:expr, $stmt:expr, $expected_sql:expr) => {{
        let (sql, params_relt) = $visitor.visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        assert_eq!(params_relt, Vec::<crate::sequel::term::param::ParamData>::new());
    }};
}

macro_rules! assert_mysql {
    ($stmt:expr, $expected_sql:expr $(, [$($params:expr),*])?) => {
        assert_dialect!(crate::sequel::visitor::visitor::mysql(), $stmt, $expected_sql $(, [$($params),*])?)
    };
}

macro_rules! assert_pg {
    ($stmt:expr, $expected_sql:expr $(, [$($params:expr),*])?) => {
        assert_dialect!(crate::sequel::visitor::visitor::postgre(), $stmt, $expected_sql $(, [$($params),*])?)
    };
}

macro_rules! assert_sqlite {
    ($stmt:expr, $expected_sql:expr $(, [$($params:expr),*])?) => {
        assert_dialect!(crate::sequel::visitor::visitor::sqlite(), $stmt, $expected_sql $(, [$($params),*])?)
    };
}

// ============================================================================
// 基础表定义
// ============================================================================
pub static USERS: LazyLock<Table> = LazyLock::new(|| Table::new("users"));
pub static ORDERS: LazyLock<Table> = LazyLock::new(|| Table::new("orders"));
pub static PRODUCTS: LazyLock<Table> = LazyLock::new(|| Table::new("products"));
pub static CATEGORIES: LazyLock<Table> = LazyLock::new(|| Table::new("categories"));
pub static COMPANIES: LazyLock<Table> = LazyLock::new(|| Table::new("companies"));

pub struct User {
    id: u32,
    username: String,
    age: u32,
    company_id: u32,
}
pub struct Order {
    id: u32,
    user_id: u32,
    created_at: DateTime,
}
pub struct Product {
    id: u32,
    name: String,
    expired_on: Date,
    category_id: u32,
}
pub struct Category {
    id: u32,
    name: String,
}
pub struct Company {
    id: u32,
    name: String,
}

impl Model for User {
    const TABLE_NAME: &'static str = "users";
}

impl Model for Order {
    const TABLE_NAME: &'static str = "orders";
}

impl Model for Product {
    const TABLE_NAME: &'static str = "products";
}

impl Model for Category {
    const TABLE_NAME: &'static str = "categories";
}

impl Model for Company {
    const TABLE_NAME: &'static str = "companies";
}

#[test]
fn test_select__all() {
    let stmt = SelectStatement::from(&*USERS);
    assert_mysql!(&stmt, "SELECT * FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT * FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT * FROM "users" AS "users0""#);
}

#[test]
fn test_select__single_column() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id"));
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
}

#[test]
fn test_select__multiple_columns() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(USERS.column("name"))
        .select(USERS.column("email"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users0`.`name`, `users0`.`email` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."name", "users0"."email" FROM "users" AS "users0""#
    );
}

#[test]
fn test_select__with_literal() {
    let stmt = SelectStatement::from(&*USERS).select(lit(1)).select(lit("hello"));
    assert_mysql!(&stmt, "SELECT 1, 'hello' FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT 1, 'hello' FROM "users" AS "users0""#);
}
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
        .select(id.clone())
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
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`id` = ? AND `users0`.`id` <> ? AND `users0`.`age` > ? AND `users0`.`age` < ? AND `users0`.`score` >= ? AND `users0`.`score` <= ? AND `users0`.`name` LIKE ? AND `users0`.`name` NOT LIKE ? AND `users0`.`country` IN (?, ?) AND `users0`.`country` NOT IN (?, ?) AND `users0`.`email` IS NOT NULL AND `users0`.`ext` IS NULL",
        [
            5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
        ]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" = $1 AND "users0"."id" <> $2 AND "users0"."age" > $3 AND "users0"."age" < $4 AND "users0"."score" >= $5 AND "users0"."score" <= $6 AND "users0"."name" LIKE $7 AND "users0"."name" NOT LIKE $8 AND "users0"."country" IN ($9, $10) AND "users0"."country" NOT IN ($11, $12) AND "users0"."email" IS NOT NULL AND "users0"."ext" IS NULL"#,
        [
            5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
        ]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" = ? AND "users0"."id" <> ? AND "users0"."age" > ? AND "users0"."age" < ? AND "users0"."score" >= ? AND "users0"."score" <= ? AND "users0"."name" LIKE ? AND "users0"."name" NOT LIKE ? AND "users0"."country" IN (?, ?) AND "users0"."country" NOT IN (?, ?) AND "users0"."email" IS NOT NULL AND "users0"."ext" IS NULL"#,
        [
            5_i64, 10_i64, 20_i64, 100_i64, 60_i64, 96_i64, "%John%", "%Lucy%", "China", "Japan", "USA", "England"
        ]
    );
}

#[test]
fn test_where__logic() {
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
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`active` = ? AND (`users0`.`role` = ? OR `users0`.`role` = ?) AND NOT (`users0`.`age` < ? OR NOT `users0`.`age` > ?)",
        [true, "admin", "superadmin", 18, 12]
    );
}

#[test]
fn test_where__complex_precedence_auto_grouping() {
    let age_limit = USERS.column("age").lt(18).or(USERS.column("age").gt(60));
    let stmt = SelectStatement::from(&*USERS).where_(age_limit.and(USERS.column("status").eq("active")));
    assert_mysql!(
        &stmt,
        "SELECT * FROM `users` AS `users0` WHERE (`users0`.`age` < ? OR `users0`.`age` > ?) AND `users0`.`status` = ?",
        [18, 60, "active"]
    );
}

#[test]
fn test_where__nested_not_precedence() {
    let condition = !USERS
        .column("status")
        .eq("pending")
        .or(USERS.column("status").eq("deleted"));

    let stmt = SelectStatement::from(&*USERS).where_(condition);

    // 因为 10 (OR) < 40 (NOT)，所以括号必须出现
    assert_mysql!(
        &stmt,
        "SELECT * FROM `users` AS `users0` WHERE NOT (`users0`.`status` = ? OR `users0`.`status` = ?)",
        ["pending", "deleted"]
    );
}

#[test]
fn test_where__null_literal() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("deleted_at").eq(Param::Null));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`deleted_at` IS NULL"
    );
}

#[test]
fn test_where__date_time_literals() {
    use crate::sequel::term::calendar::{Date, DateTime, Time};

    let date = Date::new(2024, 1, 15).unwrap();
    let time = Time::new(10, 30, 0, 0).unwrap();
    let datetime = DateTime::from(date, time).unwrap();

    let stmt = SelectStatement::from(&*ORDERS)
        .select(ORDERS.column("id"))
        .where_(ORDERS.column("created_at").eq(datetime));

    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`id` FROM `orders` AS `orders0` WHERE `orders0`.`created_at` = ?",
        [datetime]
    );
}

#[test]
fn test_where__boolean_literals() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("active").eq(true));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`active` = ?",
        [true]
    );

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("active").eq(false));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`active` = ?",
        [false]
    );
}

#[test]
fn test_where__float_literals() {
    let stmt = SelectStatement::from(&*ORDERS)
        .select(ORDERS.column("price"))
        .where_(ORDERS.column("price").eq(19.99));
    assert_mysql!(
        &stmt,
        "SELECT `orders0`.`price` FROM `orders` AS `orders0` WHERE `orders0`.`price` = ?",
        [19.99f64]
    );
}

#[test]
fn test_where__string_with_backslash() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("path"))
        .where_(USERS.column("path").eq("C:\\Users\\test"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`path` FROM `users` AS `users0` WHERE `users0`.`path` = ?",
        ["C:\\Users\\test"]
    );
}

#[test]
fn test_where__empty_select_becomes_star() {
    // 当没有指定 select 列时，应该生成 SELECT *
    let stmt = SelectStatement::from(&*USERS);
    assert_mysql!(&stmt, "SELECT * FROM `users` AS `users0`");
}

#[test]
fn test_where__string_with_quotes() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("name"))
        .where_(USERS.column("name").eq("O'Brien"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`name` FROM `users` AS `users0` WHERE `users0`.`name` = ?",
        ["O'Brien"]
    );

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("name"))
        .where_(USERS.column("name").eq(lit("O'Brien")));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`name` FROM `users` AS `users0` WHERE `users0`.`name` = 'O''Brien'"
    );
}

#[test]
fn test_where__from_binary_equivalent() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("myid").eq(5_i64).alias("anon_1"))
        .where_(USERS.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`myid` = ? AS `anon_1` FROM `users` AS `users0` WHERE `users0`.`name` = ?",
        [5, "foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."myid" = $1 AS "anon_1" FROM "users" AS "users0" WHERE "users0"."name" = $2"#,
        [5, "foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."myid" = ? AS "anon_1" FROM "users" AS "users0" WHERE "users0"."name" = ?"#,
        [5, "foo"]
    );
}

#[test]
fn test_where__collection_as_from() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(USERS.column("data"));

    assert_mysql!(&stmt, "SELECT `users0`.`id`, `users0`.`data` FROM `users` AS `users0`");
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."data" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", "users0"."data" FROM "users" AS "users0""#
    );
}

#[test]
fn test_complex_query() {
    let u = USERS.clone().alias("u");
    let o = ORDERS.clone().alias("o");
    let p = PRODUCTS.clone().alias("p");

    let stmt = SelectStatement::from(&u)
        .select(vec![
            u.column("id").into(),
            u.column("name").into(),
            sum(o.column("total")).alias("total_spent"),
            count(o.column("id")).alias("order_count"),
        ])
        .join(&o, u.column("id").eq(o.column("user_id")))
        .join(&p, o.column("product_id").eq(p.column("id")))
        .where_(o.column("total").gt(100).and(p.column("category_id").eq(5)))
        .distinct()
        .limit(10)
        .offset(20);

    assert_mysql!(
        &stmt,
        "SELECT DISTINCT `u`.`id`, `u`.`name`, SUM(`o`.`total`) AS `total_spent`, COUNT(`o`.`id`) AS `order_count` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id` INNER JOIN `products` AS `p` ON `o`.`product_id` = `p`.`id` WHERE `o`.`total` > ? AND `p`.`category_id` = ? LIMIT 10 OFFSET 20",
        [100i64, 5i64]
    );
}

#[test]
fn test_distinct() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("city")).distinct();
    assert_mysql!(&stmt, "SELECT DISTINCT `users0`.`city` FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT DISTINCT "users0"."city" FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT DISTINCT "users0"."city" FROM "users" AS "users0""#);
}

#[test]
fn test_distinct__on() {
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
#[test]
fn test_func__count_all() {
    let stmt = SelectStatement::from(&*USERS).select(count_all());
    assert_mysql!(&stmt, "SELECT COUNT(*) FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT COUNT(*) FROM "users" AS "users0""#);
}

#[test]
fn test_func__count_column() {
    let c = USERS.column("email");
    let stmt = SelectStatement::from(&*USERS).select(count(c));
    assert_mysql!(&stmt, "SELECT COUNT(`users0`.`email`) FROM `users` AS `users0`");
    assert_pg!(&stmt, r#"SELECT COUNT("users0"."email") FROM "users" AS "users0""#);
    assert_sqlite!(&stmt, r#"SELECT COUNT("users0"."email") FROM "users" AS "users0""#);
}

#[test]
fn test_func__count_distinct() {
    let stmt = SelectStatement::from(&*USERS).select(count(USERS.column("city")).distinct());
    assert_mysql!(&stmt, "SELECT COUNT(DISTINCT `users0`.`city`) FROM `users` AS `users0`");
    assert_pg!(
        &stmt,
        r#"SELECT COUNT(DISTINCT "users0"."city") FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COUNT(DISTINCT "users0"."city") FROM "users" AS "users0""#
    );
}

#[test]
fn test_func__count_distinct_multiple() {
    let stmt = SelectStatement::from(&*USERS)
        .select(count([USERS.column("city"), USERS.column("username"), USERS.column("id")]).distinct())
        .select(USERS.column("id"));
    assert_mysql!(
        &stmt,
        "SELECT COUNT(DISTINCT `users0`.`city`, `users0`.`username`, `users0`.`id`), `users0`.`id` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COUNT(DISTINCT ("users0"."city", "users0"."username", "users0"."id")), "users0"."id" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COUNT(*), "sq0"."id" FROM (SELECT DISTINCT "users0"."city", "users0"."username", "users0"."id", "users0"."id" FROM "users" AS "users0") AS "sq0""#
    );
}

#[test]
fn test_func__abs_ceil_floor() {
    let stmt = SelectStatement::from(&*ORDERS)
        .select(sum(ORDERS.column("total")))
        .select(avg(ORDERS.column("price")))
        .select(max(ORDERS.column("total")))
        .select(min(ORDERS.column("price")))
        .select(abs(ORDERS.column("discount")))
        .select(ceil(ORDERS.column("price")))
        .select(floor(ORDERS.column("tax")))
        .select(lower(ORDERS.column("name")))
        .select(upper(ORDERS.column("brand_name")))
        .select(sqrt(ORDERS.column("quantity")));
    assert_mysql!(
        &stmt,
        "SELECT SUM(`orders0`.`total`), AVG(`orders0`.`price`), MAX(`orders0`.`total`), MIN(`orders0`.`price`), ABS(`orders0`.`discount`), CEIL(`orders0`.`price`), FLOOR(`orders0`.`tax`), LOWER(`orders0`.`name`), UPPER(`orders0`.`brand_name`), SQRT(`orders0`.`quantity`) FROM `orders` AS `orders0`"
    );
}

#[test]
fn test_func__custom_func() {
    let stmt = SelectStatement::from(&*USERS).select(func(
        "CONCAT",
        vec![USERS.column("first_name"), USERS.column("last_name")],
    ));
    assert_mysql!(
        &stmt,
        "SELECT CONCAT(`users0`.`first_name`, `users0`.`last_name`) FROM `users` AS `users0`"
    );
}

#[test]
fn test_func__coalesce() {
    let stmt = SelectStatement::from(&*USERS).select(coalesce(vec![
        Expr::Column(USERS.column("email")),
        lit("no-email").into(),
    ]));
    assert_mysql!(
        &stmt,
        "SELECT COALESCE(`users0`.`email`, 'no-email') FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", 'no-email') FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", 'no-email') FROM "users" AS "users0""#
    );
}

#[test]
fn test_func__coalesce_multiple() {
    let stmt = SelectStatement::from(&*USERS).select(coalesce(vec![
        Expr::Column(USERS.column("email")),
        Expr::Column(USERS.column("phone")),
        lit("no-contact").into(),
    ]));
    assert_mysql!(
        &stmt,
        "SELECT COALESCE(`users0`.`email`, `users0`.`phone`, 'no-contact') FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", "users0"."phone", 'no-contact') FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COALESCE("users0"."email", "users0"."phone", 'no-contact') FROM "users" AS "users0""#
    );
}

#[test]
fn test_func__from_func_not_the_first_arg_equivalent() {
    let stmt = SelectStatement::from(&*USERS)
        .select(func("bar", vec![lit(true).into(), Expr::from(USERS.column("myid"))]).alias("bar_1"))
        .where_(USERS.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT BAR(1, `users0`.`myid`) AS `bar_1` FROM `users` AS `users0` WHERE `users0`.`name` = ?",
        ["foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT BAR(true, "users0"."myid") AS "bar_1" FROM "users" AS "users0" WHERE "users0"."name" = $1"#,
        ["foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT BAR(1, "users0"."myid") AS "bar_1" FROM "users" AS "users0" WHERE "users0"."name" = ?"#,
        ["foo"]
    );
}

#[test]
fn test_join() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(ORDERS.column("total"))
        .select(PRODUCTS.column("name"))
        .join(&*ORDERS, USERS.column("id").eq(ORDERS.column("user_id")))
        .left_join(&*PRODUCTS, ORDERS.column("id").eq(PRODUCTS.column("order_id")))
        .right_join(&*CATEGORIES, CATEGORIES.column("id").eq(PRODUCTS.column("category_id")))
        .full_join(&*COMPANIES, COMPANIES.column("id").eq(USERS.column("company_id")));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `orders0`.`total`, `products0`.`name` FROM `users` AS `users0` INNER JOIN `orders` AS `orders0` ON `users0`.`id` = `orders0`.`user_id` LEFT JOIN `products` AS `products0` ON `orders0`.`id` = `products0`.`order_id` RIGHT JOIN `categories` AS `categories0` ON `categories0`.`id` = `products0`.`category_id` FULL JOIN `companies` AS `companies0` ON `companies0`.`id` = `users0`.`company_id`"
    );
}
#[test]
fn test_join__cross_join() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(PRODUCTS.column("name"))
        .cross_join(&*PRODUCTS);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `products0`.`name` FROM `users` AS `users0` CROSS JOIN `products` AS `products0`"
    );
}
#[test]
fn test_join__cross_join_with_same_table() {
    let u_fork = USERS.fork();
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(u_fork.column("name"))
        .cross_join(u_fork);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users1`.`name` FROM `users` AS `users0` CROSS JOIN `users` AS `users1`"
    );

    let u_clone = USERS.clone();
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(u_clone.column("name"))
        .cross_join(u_clone);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users0`.`name` FROM `users` AS `users0` CROSS JOIN `users` AS `users0`"
    );
}

#[test]
fn test_limit__0() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(0);
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
}

#[test]
fn test_limit__only() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).limit(10);
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0` LIMIT 10");
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10"#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" LIMIT 10"#);
}

#[test]
fn test_limit__offset_only() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("id")).offset(20);
    // MySQL 不支持单独 OFFSET
    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0`");
    // PostgreSQL 和 SQLite 支持
    assert_pg!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" OFFSET 20"#);
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0" OFFSET 20"#);
}

#[test]
fn test_limit__limit_and_offset() {
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

#[test]
fn test_iteral__arithmetic_operations() {
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
fn test_iteral__addition_subtraction() {
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
fn test_iteral__division_modulo() {
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
fn test_iteral__mixed_arithmetic() {
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

#[test]
fn test_subquery__exists_subquery() {
    let where_clause = ORDERS.column("user_id").eq(USERS.column("id"));
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(exists(SelectStatement::from(&*ORDERS).where_(where_clause)));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE EXISTS((SELECT * FROM `orders` AS `orders0` WHERE `orders0`.`user_id` = `users0`.`id`))"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS((SELECT * FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id"))"#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS((SELECT * FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id"))"#
    );
}

#[test]
fn test_subquery__scalar_subquery() {
    let subquery = SelectStatement::from(&*ORDERS)
        .select(max(ORDERS.column("total")))
        .where_(ORDERS.column("user_id").eq(USERS.column("id")));

    let stmt = SelectStatement::from(&*USERS).select(vec![USERS.column("id").into(), Expr::from(subquery)]);

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, (SELECT MAX(`orders0`.`total`) FROM `orders` AS `orders0` WHERE `orders0`.`user_id` = `users0`.`id`) FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id", (SELECT MAX("orders0"."total") FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id") FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id", (SELECT MAX("orders0"."total") FROM "orders" AS "orders0" WHERE "orders0"."user_id" = "users0"."id") FROM "users" AS "users0""#
    );
}

#[test]
fn test_subquery__in_subquery() {
    let subquery = SelectStatement::from(&*ORDERS).select(ORDERS.column("user_id"));

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(USERS.column("id").in_(vec![subquery]));

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`id` IN ((SELECT `orders0`.`user_id` FROM `orders` AS `orders0`))"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" IN ((SELECT "orders0"."user_id" FROM "orders" AS "orders0"))"#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE "users0"."id" IN ((SELECT "orders0"."user_id" FROM "orders" AS "orders0"))"#
    );
}

#[test]
fn test_subquery__nested_subquery_with_join() {
    let subquery = SelectStatement::from(&*ORDERS)
        .select(max(ORDERS.column("total")))
        .where_(ORDERS.column("status").eq("completed"));

    let stmt = SelectStatement::from(&*USERS)
        .select(vec![
            USERS.column("id").into(),
            Expr::from(subquery.clone()).alias("max_order"),
        ])
        .where_(subquery.gt(1000));

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, (SELECT MAX(`orders0`.`total`) FROM `orders` AS `orders0` WHERE `orders0`.`status` = ?) AS `max_order` FROM `users` AS `users0` WHERE (SELECT MAX(`orders0`.`total`) FROM `orders` AS `orders0` WHERE `orders0`.`status` = ?) > ?",
        ["completed", "completed", 1000i64]
    );
}
#[test]
fn test_alias__column_alias() {
    let stmt = SelectStatement::from(&*USERS).select(USERS.column("name").alias("username"));
    assert_mysql!(&stmt, "SELECT `users0`.`name` AS `username` FROM `users` AS `users0`");
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."name" AS "username" FROM "users" AS "users0""#
    );
}

#[test]
fn test_alias__expression_alias() {
    let stmt = SelectStatement::from(&*USERS)
        .select((USERS.column("first_name") + USERS.column("last_name").clone()).alias("full_name"));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`first_name` + `users0`.`last_name` AS `full_name` FROM `users` AS `users0`"
    );
}

#[test]
fn test_alias__func_alias() {
    let stmt = SelectStatement::from(&*USERS).select(upper(USERS.column("name")).alias("upper_name"));
    assert_mysql!(
        &stmt,
        "SELECT UPPER(`users0`.`name`) AS `upper_name` FROM `users` AS `users0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT UPPER("users0"."name") AS "upper_name" FROM "users" AS "users0""#
    );
}

#[test]
fn test_alias__table_alias() {
    let u = USERS.clone().alias("u");
    let o = ORDERS.clone().alias("o");

    let stmt = SelectStatement::from(&u)
        .select(vec![u.column("id"), o.column("total")])
        .join(&o, u.column("id").eq(o.column("user_id")));

    assert_mysql!(
        &stmt,
        "SELECT `u`.`id`, `o`.`total` FROM `users` AS `u` INNER JOIN `orders` AS `o` ON `u`.`id` = `o`.`user_id`"
    );
}

#[test]
fn test_locker__for_update() {
    use crate::sequel::term::lock::{Lock, Wait};

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .for_update()
        .no_wait();

    assert_mysql!(&stmt, "SELECT `users0`.`id` FROM `users` AS `users0` FOR UPDATE NOWAIT");
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" FOR UPDATE NOWAIT"#
    );
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
}

#[test]
fn test_locker__for_share() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .for_share()
        .skip();

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` FOR SHARE SKIP LOCKED"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" FOR SHARE SKIP LOCKED"#
    );
    assert_sqlite!(&stmt, r#"SELECT "users0"."id" FROM "users" AS "users0""#);
}

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
//         "SELECT `users0`.`id` FROM `users` AS `users0` FORCE INDEX (idx_users_email)"
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
