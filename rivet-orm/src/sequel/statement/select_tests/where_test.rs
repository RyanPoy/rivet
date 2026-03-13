use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::USERS;

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
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE `users0`.`active` = ? AND (`users0`.`role` = ? OR `users0`.`role` = ?) AND NOT (`users0`.`age` < ? OR NOT `users0`.`age` > ?)",
        [true, "admin", "superadmin", 18, 12]
    );
}

#[test]
fn test_where_complex_precedence_auto_grouping() {
    let age_limit = USERS.column("age").lt(18).or(USERS.column("age").gt(60));
    let stmt = SelectStatement::from(&*USERS).where_(age_limit.and(USERS.column("status").eq("active")));
    assert_mysql!(
        &stmt,
        "SELECT * FROM `users` AS `users0` WHERE (`users0`.`age` < ? OR `users0`.`age` > ?) AND `users0`.`status` = ?",
        [18, 60, "active"]
    );
}

#[test]
fn test_where_nested_not_precedence() {
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
