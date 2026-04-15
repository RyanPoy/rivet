use crate::prelude::*;
use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::*;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::func;
use crate::sequel::term::param::lit;

#[test]
fn test_filter_by_from_func_not_the_first_arg_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(func("bar", vec![lit(true).into(), Expr::from(MYTABLE.column("myid"))]).alias("bar_1"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT BAR(1, `mytable0`.`myid`) AS `bar_1` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        ["foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT BAR(true, "mytable0"."myid") AS "bar_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $1"#,
        ["foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT BAR(1, "mytable0"."myid") AS "bar_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        ["foo"]
    );
}

#[test]
fn test_filter_by_from_binary_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid").eq(5_i64).alias("anon_1"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid` = ? AS `anon_1` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        [5, "foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid" = $1 AS "anon_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $2"#,
        [5, "foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid" = ? AS "anon_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        [5, "foo"]
    );
}

#[test]
fn test_methods_generative_having() {
    let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    let s2 = s1.clone().where_(USERS.column("id").eq(1_i64));
    assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

#[test]
#[ignore = "rivet-orm 可能不支持 order_by() 方法"]
fn test_methods_generative_order_by() {}

#[test]
#[ignore = "rivet-orm 可能不支持 group_by() 方法"]
fn test_methods_generative_group_by() {}

#[test]
fn test_c_collection_as_from() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0`"
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0""#
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0""#
    );
}
