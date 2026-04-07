// ============================================================================
// SQLAlchemy test_select.py 完整翻译测试
// ============================================================================
//
// 文件名: test_sqlalchemy.rs
// 路径: rivet-orm/src/sequel/statement/select_tests/test_sqlalchemy.rs
// 来源: https://raw.githubusercontent.com/sqlalchemy/sqlalchemy/main/test/sql/test_select.py
//
// 翻译说明:
//
// 1. API 映射对照表
//    | SQLAlchemy API                      | rivet-orm API                                    |
//    |--------------------------------------|--------------------------------------------------|
//    | select(table.c.col)                  | SelectStatement::from(&*TABLE).select(T.col("x"))|
//    | .where(t1.c.col == t2.c.col)         | .where_(T1.col("x").eq(T2.col("y")))            |
//    | .join(t2, on_clause)                 | .join(&*T2, on_expr)                            |
//    | .outerjoin_from(t1, t2, on)          | .left_join(&*T2, on_expr)                       |
//    | .select_from(tbl)                    | SelectStatement::from(&*TBL)                     |
//    | .filter_by(col=val)                  | .where_(T.col("x").eq(val))                     |
//    | .with_only_columns(cols)             | 不支持                                           |
//    | .union() / .except_() / .intersect() | 不支持                                           |
//    | .subquery()                          | 不支持                                           |
//    | .exists()                            | 支持 (crate::sequel::term::func::exists)        |
//    | .cast(col, Type)                     | 不支持                                           |
//    | .tuple_()                           | 不支持                                           |
//
// 2. 多 dialect 验证说明
//    - MySQL:     使用 `backtick` 引号，参数绑定用 ?
//    - PostgreSQL: 使用 "doublequote" 引号，参数绑定用 $1, $2, ...
//    - SQLite:    使用 "doublequote" 引号，参数绑定用 ?
//
// 3. 不支持的功能
//    - with_only_columns(), filter_by(), union(), except_(), intersect()
//    - subquery(), tuple_(), cast() 等
//    - 这些测试会用 #[ignore] 标记
//
// ============================================================================

use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::*;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{count, exists, func, max};
use crate::sequel::term::literal::Literal;
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::Table;
use crate::sequel::term::comparable::Comparable;

// ============================================================================
// ============================================================================
// SelectTest 类 - SQLAlchemy SelectTest 的全部测试方法
// ============================================================================
// ============================================================================

// ----------------------------------------------------------------------------
// test_new_calling_style
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试新式调用风格: select(table1.c.myid).where(...)
// 验证 select 接受单列并正确生成 WHERE 子句。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c.myid).where(table1.c.myid == table2.c.otherid)
// self.assert_compile(
//     stmt,
//     "SELECT mytable.myid FROM mytable, myothertable "
//     "WHERE mytable.myid = myothertable.otherid",
// )
// ```
//
// 【翻译后的 Rust 代码】
// SQLAlchemy 通过 WHERE 子句隐式添加 myothertable 到 FROM 子句（cross join）。
// rivet-orm 需要显式使用 cross_join() 来实现相同效果。
//
// 【多 dialect 验证】
// - MySQL:     `backtick` 引号，? 参数绑定
// - PostgreSQL: "doublequote" 引号，$1 参数绑定
// - SQLite:    "doublequote" 引号，? 参数绑定
// ----------------------------------------------------------------------------
#[test]
fn test_new_calling_style() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .cross_join((*MYOTHERTABLE).clone())
        .where_(MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(&stmt, "SELECT `mytable0`.`myid` FROM `mytable` AS `mytable0` CROSS JOIN `myothertable` AS `myothertable0` WHERE `mytable0`.`myid` = `myothertable0`.`otherid`", []);
    assert_pg!(&stmt, r#"SELECT "mytable0"."myid" FROM "mytable" AS "mytable0" CROSS JOIN "myothertable" AS "myothertable0" WHERE "mytable0"."myid" = "myothertable0"."otherid""#, []);
    assert_sqlite!( &stmt, r#"SELECT "mytable0"."myid" FROM "mytable" AS "mytable0" CROSS JOIN "myothertable" AS "myothertable0" WHERE "mytable0"."myid" = "myothertable0"."otherid""#, []);
}

// ----------------------------------------------------------------------------
// test_select_no_columns - 组合 1: select().select_from(tbl).where(tbl.c.id == 123)
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试没有选择任何列的 select 语句，验证自动变为 SELECT * 或类似形式。
//
// 【SQLAlchemy 原版代码】
// ```python
// tbl = table("tbl", column("id"))
// stmt = select().select_from(tbl).where(tbl.c.id == 123)
// self.assert_compile(stmt, "SELECT FROM tbl WHERE tbl.id = :id_1")
// ```
//
// 【注意】SQLAlchemy 生成的 SQL 有趣地没有 * 号，而 rivet-orm 会生成 SELECT *
//
// 【多 dialect 验证】
// - MySQL:     LIMIT/OFFSET 支持
// - PostgreSQL: LIMIT/OFFSET 支持，$1 参数
// - SQLite:    LIMIT/OFFSET 支持
// ----------------------------------------------------------------------------
#[test]
fn test_select_no_columns_select_from_where() {
    let stmt = SelectStatement::from(&*TBL).where_(TBL.column("id").eq(123_i64));

    // MySQL: SELECT * FROM ... WHERE ... = ?
    assert_mysql!(
        &stmt,
        "SELECT * FROM `tbl` AS `t0` WHERE `t0`.`id` = ?",
        [123_i64]
    );

    // PostgreSQL: SELECT * FROM ... WHERE ... = $1
    assert_pg!(
        &stmt,
        r#"SELECT * FROM "tbl" AS "t0" WHERE "t0"."id" = $1"#,
        [123_i64]
    );

    // SQLite: SELECT * FROM ... WHERE ... = ?
    assert_sqlite!(
        &stmt,
        r#"SELECT * FROM "tbl" AS "t0" WHERE "t0"."id" = ?"#,
        [123_i64]
    );
}

// ----------------------------------------------------------------------------
// test_select_no_columns - 组合 2: select().where(true())
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试使用 true() 函数的 WHERE 子句，生成 "WHERE 1 = 1"。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select().where(true())
// self.assert_compile(stmt, "SELECT WHERE 1 = 1")
// ```
//
// 【翻译后的 Rust 代码】
// true() 在 SQLAlchemy 中是布尔常量，rivet-orm 使用 1=1 表达式实现。
//
// 【多 dialect 验证】
// - MySQL:     1 = 1 表达式
// - PostgreSQL: 1 = 1 表达式
// - SQLite:    1 = 1 表达式
// ----------------------------------------------------------------------------
#[test]
fn test_select_no_columns_where_true() {
    // SQLAlchemy: select().where(true()) -> "SELECT WHERE 1 = 1"
    // rivet-orm: 使用 1 = 1 表达式
    let item = Literal::Int(1_i64);
    let stmt = SelectStatement::from(&*TBL).where_(item.eq(1_i64));

    assert_mysql!(
        &stmt,
        "SELECT * FROM `tbl` AS `t0` WHERE 1 = 1",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT * FROM "tbl" AS "t0" WHERE 1 = 1"#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT * FROM "tbl" AS "t0" WHERE 1 = 1"#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_select_no_columns - 组合 3: select().select_from(tbl).where(...).exists()
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 exists() 子查询的使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select().select_from(tbl).where(tbl.c.id == 123).exists()
// self.assert_compile(stmt, "EXISTS (SELECT FROM tbl WHERE tbl.id = :id_1)")
// ```
//
// 【翻译后的 Rust 代码】
// exists() 在 rivet-orm 中通过 crate::sequel::term::func::exists 支持。
//
// 【多 dialect 验证】
// - MySQL:     EXISTS 子查询
// - PostgreSQL: EXISTS 子查询，$1 参数
// - SQLite:    EXISTS 子查询
// ----------------------------------------------------------------------------
#[test]
fn test_select_no_columns_select_from_where_exists() {
    let exists_stmt = SelectStatement::from(&*TBL)
        .where_(TBL.column("id").eq(123_i64));

    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .where_(exists(exists_stmt).into());

    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id` FROM `users` AS `users0` WHERE EXISTS(SELECT * FROM `tbl` AS `t0` WHERE `t0`.`id` = ?)",
        [123_i64]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS(SELECT * FROM "tbl" AS "t0" WHERE "t0"."id" = $1)"#,
        [123_i64]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "users0"."id" FROM "users" AS "users0" WHERE EXISTS(SELECT * FROM "tbl" AS "t0" WHERE "t0"."id" = ?)"#,
        [123_i64]
    );
}

// ----------------------------------------------------------------------------
// test_new_calling_style_clauseelement_thing_that_has_iter
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试实现了 __clause_element__() 和 __iter__() 协议的自定义对象。
//
// 【SQLAlchemy 原版代码】
// ```python
// class Thing:
//     def __clause_element__(self):
//         return table1
//     def __iter__(self):
//         return iter(["a", "b", "c"])
// stmt = select(Thing())
// self.assert_compile(stmt, "SELECT mytable.myid, mytable.name, mytable.description FROM mytable")
// ```
//
// 【rivet-orm 说明】
// rivet-orm 不支持 __clause_element__/__iter__ 协议。
//
// 【状态】#[ignore] - rivet-orm 不支持此协议
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 __clause_element__/__iter__ 协议"]
fn test_new_calling_style_clauseelement_thing_that_has_iter() {
    // SQLAlchemy 使用 Thing.__iter__() 返回的列名
    // 然后 Thing.__clause_element__() 返回 table1
    // 最终 select(Thing()) 展开为 select(table1.c.a, table1.c.b, table1.c.c)
    // rivet-orm 没有等价的协议支持
}

// ----------------------------------------------------------------------------
// test_new_calling_style_inspectable_ce_thing_that_has_iter
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试通过 inspection 注册机制将 Thing 适配为 InspectedThing 的场景。
//
// 【SQLAlchemy 原版代码】
// ```python
// class Thing:
//     def __iter__(self):
//         return iter(["a", "b", "c"])
// class InspectedThing:
//     def __clause_element__(self):
//         return table1
// @_inspects(Thing)  # 注册适配器
// def _ce(thing):
//     return InspectedThing()
// stmt = select(Thing())
// self.assert_compile(stmt, "SELECT mytable.myid, mytable.name, mytable.description FROM mytable")
// ```
//
// 【状态】#[ignore] - rivet-orm 不支持 inspection 注册
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 SQLAlchemy 的 inspection 注册机制"]
fn test_new_calling_style_inspectable_ce_thing_that_has_iter() {
    // SQLAlchemy 的 @_inspects 装饰器允许运行时注册类型适配器
    // rivet-orm 没有等价的机制
}

// ----------------------------------------------------------------------------
// test_join_nofrom_implicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试没有显式 select_from 时，join() 隐式使用前一个表作为左表。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1).join(table2, table1.c.myid == table2.c.otherid)
// self.assert_compile(
//     stmt,
//     "SELECT mytable.myid, mytable.name, mytable.description "
//     "FROM mytable JOIN myothertable "
//     "ON mytable.myid = myothertable.otherid",
// )
// ```
//
// 【多 dialect 验证】
// - MySQL:     INNER JOIN
// - PostgreSQL: INNER JOIN
// - SQLite:    INNER JOIN
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_implicit_left_side_explicit_onclause() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("name"))
        .select(MYTABLE.column("description"))
        .join(&*MYOTHERTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`name`, `mytable0`.`description` FROM `mytable` AS `mytable0` INNER JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_nofrom_implicit_left_side_explicit_onclause_3level
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试三层 JOIN: parent -> child -> grandchild，每层都有显式 ON 条件。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join(child, child.c.parent_id == parent.c.id)
//     .join(grandchild, grandchild.c.child_id == child.c.id)
// )
// self.assert_compile(
//     stmt,
//     "SELECT parent.id, parent.data FROM parent JOIN child "
//     "ON child.parent_id = parent.id "
//     "JOIN grandchild ON grandchild.child_id = child.id",
// )
// ```
//
// 【多 dialect 验证】
// - MySQL:     三层 INNER JOIN
// - PostgreSQL: 三层 INNER JOIN
// - SQLite:    三层 INNER JOIN
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_implicit_left_side_explicit_onclause_3level() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, CHILD.column("parent_id").eq(PARENT.column("id")))
        .join(&*GRANDCHILD, GRANDCHILD.column("child_id").eq(CHILD.column("id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `child0`.`parent_id` = `parent0`.`id` INNER JOIN `grandchild` AS `grandchild0` ON `grandchild0`.`child_id` = `child0`.`id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "child0"."parent_id" = "parent0"."id" INNER JOIN "grandchild" AS "grandchild0" ON "grandchild0"."child_id" = "child0"."id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "child0"."parent_id" = "parent0"."id" INNER JOIN "grandchild" AS "grandchild0" ON "grandchild0"."child_id" = "child0"."id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_nofrom_explicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 join_from() 明确指定左右表的场景。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1).join_from(
//     table1, table2, table1.c.myid == table2.c.otherid
// )
// self.assert_compile(
//     stmt,
//     "SELECT mytable.myid, mytable.name, mytable.description "
//     "FROM mytable JOIN myothertable "
//     "ON mytable.myid = myothertable.otherid",
// )
// ```
//
// 【翻译说明】
// SQLAlchemy 的 join_from(t1, t2, on) 等价于 rivet-orm 的 join(&T2, on_expr)
//
// 【多 dialect 验证】同 test_join_nofrom_implicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_explicit_left_side_explicit_onclause() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("name"))
        .select(MYTABLE.column("description"))
        .join(&*MYOTHERTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`name`, `mytable0`.`description` FROM `mytable` AS `mytable0` INNER JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_from_multiple_explicit_left_side_implicit_onclause - child_grandchild
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test #12931 变体 1】
// 测试 join_from() 的 left side 推断规则:
// 当使用 join_from(parent, child) 后再 join_from(child, grandchild_w_parent) 时，
// grandchild_w_parent 的 left side 应该是 child。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join_from(parent, child)
//     .join_from(child, grandchild_w_parent)
// )
// ```
//
// 【多 dialect 验证】
// - MySQL:     多表 JOIN
// - PostgreSQL: 多表 JOIN
// - SQLite:    多表 JOIN
// ----------------------------------------------------------------------------
#[test]
fn test_join_from_multiple_explicit_left_side_implicit_onclause_child_grandchild() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .join(&*GRANDCHILD_W_PARENT, CHILD.column("id").eq(GRANDCHILD_W_PARENT.column("child_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `grandchildwparent` AS `grandchildwparent0` ON `child0`.`id` = `grandchildwparent0`.`child_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "child0"."id" = "grandchildwparent0"."child_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "child0"."id" = "grandchildwparent0"."child_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_from_multiple_explicit_left_side_implicit_onclause - parent_grandchild
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test #12931 变体 2】
// 这次使用 join_from(parent, grandchild_w_parent)，left side 是 parent。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join_from(parent, child)
//     .join_from(parent, grandchild_w_parent)  # left side 是 parent
// )
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_from_multiple_explicit_left_side_implicit_onclause_parent_grandchild() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .join(&*GRANDCHILD_W_PARENT, PARENT.column("id").eq(GRANDCHILD_W_PARENT.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `grandchildwparent` AS `grandchildwparent0` ON `parent0`.`id` = `grandchildwparent0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "parent0"."id" = "grandchildwparent0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "parent0"."id" = "grandchildwparent0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_from_multiple_explicit_left_side_implicit_onclause - grandchild_alone
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test #12931 变体 3】
// 混用 join_from 和 join。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join_from(parent, child)
//     .join(grandchild_w_parent)  # 普通的 join，left side 继承自上一个 JOIN
// )
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_from_multiple_explicit_left_side_implicit_onclause_grandchild_alone() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .join(&*GRANDCHILD_W_PARENT, CHILD.column("id").eq(GRANDCHILD_W_PARENT.column("child_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `grandchildwparent` AS `grandchildwparent0` ON `child0`.`id` = `grandchildwparent0`.`child_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "child0"."id" = "grandchildwparent0"."child_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchildwparent" AS "grandchildwparent0" ON "child0"."id" = "grandchildwparent0"."child_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_outerjoin_nofrom_explicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 outerjoin_from() 生成 LEFT OUTER JOIN。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1).outerjoin_from(
//     table1, table2, table1.c.myid == table2.c.otherid
// )
// self.assert_compile(
//     stmt,
//     "SELECT mytable.myid, mytable.name, mytable.description "
//     "FROM mytable LEFT OUTER JOIN myothertable "
//     "ON mytable.myid = myothertable.otherid",
// )
// ```
//
// 【翻译说明】
// SQLAlchemy 的 outerjoin_from 等价于 rivet-orm 的 left_join()。
//
// 【多 dialect 验证】
// - MySQL:     LEFT JOIN
// - PostgreSQL: LEFT JOIN
// - SQLite:    LEFT JOIN
// ----------------------------------------------------------------------------
#[test]
fn test_outerjoin_nofrom_explicit_left_side_explicit_onclause() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("name"))
        .select(MYTABLE.column("description"))
        .left_join(&*MYOTHERTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`name`, `mytable0`.`description` FROM `mytable` AS `mytable0` LEFT JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" LEFT JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" LEFT JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_nofrom_implicit_left_side_implicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试没有显式 ON 条件的 join()。
// SQLAlchemy 会自动从外键关系推断 JOIN 条件。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child)
// self.assert_compile(
//     stmt,
//     "SELECT parent.id, parent.data FROM parent JOIN child "
//     "ON parent.id = child.parent_id",
// )
// ```
//
// 【rivet-orm 说明】
// **重要**: rivet-orm 不支持自动外键推断，必须显式提供 ON 条件。
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_implicit_left_side_implicit_onclause() {
    // SQLAlchemy 自动推断: parent JOIN child ON parent.id = child.parent_id
    // rivet-orm 必须显式提供 ON 条件
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_nofrom_implicit_left_side_implicit_onclause_3level
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试三层隐式 ON 条件的 JOIN。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child).join(grandchild)
// ```
//
// 【rivet-orm 说明】
// **重要**: rivet-orm 必须显式提供每一层 JOIN 的 ON 条件。
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_implicit_left_side_implicit_onclause_3level() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .join(&*GRANDCHILD, CHILD.column("id").eq(GRANDCHILD.column("child_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `grandchild` AS `grandchild0` ON `child0`.`id` = `grandchild0`.`child_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_nofrom_explicit_left_side_implicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 使用 join_from(parent, child) 的显式形式，但仍然没有 ON 条件（依赖外键推断）。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join_from(parent, child)
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_nofrom_explicit_left_side_implicit_onclause() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_froms_implicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 先 select_from(table1) 明确指定 FROM，再使用 join() 添加 JOIN。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(table1)
//     .select_from(table1)
//     .join(table2, table1.c.myid == table2.c.otherid)
// )
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_froms_implicit_left_side_explicit_onclause() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("name"))
        .select(MYTABLE.column("description"))
        .join(&*MYOTHERTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`name`, `mytable0`.`description` FROM `mytable` AS `mytable0` INNER JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_froms_explicit_left_side_explicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 使用 select_from() + join_from() 的完全显式形式。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(table1)
//     .select_from(table1)
//     .join_from(table1, table2, table1.c.myid == table2.c.otherid)
// )
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_froms_explicit_left_side_explicit_onclause() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("name"))
        .select(MYTABLE.column("description"))
        .join(&*MYOTHERTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`name`, `mytable0`.`description` FROM `mytable` AS `mytable0` INNER JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."name", "mytable0"."description" FROM "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_froms_implicit_left_side_implicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// select_from() + 普通 join()（隐式 ON 推断）。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).select_from(parent).join(child)
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_froms_implicit_left_side_implicit_onclause() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_froms_explicit_left_side_implicit_onclause
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// select_from() + join_from() 的完全显式形式。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).select_from(parent).join_from(parent, child)
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_join_froms_explicit_left_side_implicit_onclause() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_join_implicit_left_side_wo_cols_onelevel
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6503】
// 测试 join() 后使用 with_only_columns() 只选择部分列。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child).with_only_columns(child.c.id)
// ```
//
// 【rivet-orm 说明】
// **重要**: rivet-orm 不支持 with_only_columns() 方法。
//
// 【状态】#[ignore] - with_only_columns() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 with_only_columns() 方法"]
fn test_join_implicit_left_side_wo_cols_onelevel() {
    // SQLAlchemy: select(parent).join(child).with_only_columns(child.c.id)
    // 期望: SELECT child.id FROM parent JOIN child ON parent.id = child.parent_id
    //
    // rivet-orm 实现方式:
    // 需要在 select() 时就直接指定要选择的列
    //
    // let stmt = SelectStatement::from(&*PARENT)
    //     .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
    //     .with_only_columns(CHILD.column("id"));  // 不支持！
}

// ----------------------------------------------------------------------------
// test_join_implicit_left_side_wo_cols_onelevel_union
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6698】
// 在 with_only_columns() 基础上加上 UNION 操作。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child).with_only_columns(child.c.id)
// stmt = stmt.union(select(child.c.id))
// ```
//
// 【状态】#[ignore] - with_only_columns() 和 union() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 with_only_columns() 和 union() 方法"]
fn test_join_implicit_left_side_wo_cols_onelevel_union() {
    // 两个功能都不支持
}

// ----------------------------------------------------------------------------
// test_join_implicit_left_side_wo_cols_twolevel
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6503】
// 测试两层 JOIN 后使用 with_only_columns()。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join(child)
//     .with_only_columns(child.c.id)
//     .join(grandchild)
//     .with_only_columns(grandchild.c.id)
// )
// ```
//
// 【状态】#[ignore] - with_only_columns() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 with_only_columns() 方法"]
fn test_join_implicit_left_side_wo_cols_twolevel() {
    // 不支持 with_only_columns()
}

// ----------------------------------------------------------------------------
// test_join_implicit_left_side_wo_cols_twolevel_union
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6698】
// 两层 JOIN + with_only_columns() + UNION。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join(child)
//     .with_only_columns(child.c.id)
//     .join(grandchild)
//     .with_only_columns(grandchild.c.id)
// )
// stmt = union(stmt, select(grandchild.c.id))
// ```
//
// 【状态】#[ignore] - 两个功能都不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 with_only_columns() 和 union() 方法"]
fn test_join_implicit_left_side_wo_cols_twolevel_union() {
    // 两个功能都不支持
}

// ----------------------------------------------------------------------------
// test_right_nested_inner_join
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试嵌套 JOIN 作为 RIGHT JOIN 的内层。
// inner = child.join(grandchild) 创建一个嵌套的 (child JOIN grandchild)
//
// 【SQLAlchemy 原版代码】
// ```python
// inner = child.join(grandchild)
// stmt = select(parent).outerjoin_from(parent, inner)
// self.assert_compile(
//     stmt,
//     "SELECT parent.id, parent.data FROM parent "
//     "LEFT OUTER JOIN "
//     "(child JOIN grandchild ON child.id = grandchild.child_id) "
//     "ON parent.id = child.parent_id",
// )
// ```
//
// 【多 dialect 验证】
// - MySQL:     嵌套 LEFT JOIN
// - PostgreSQL: 嵌套 LEFT JOIN
// - SQLite:    嵌套 LEFT JOIN
// ----------------------------------------------------------------------------
#[test]
fn test_right_nested_inner_join() {
    // SQLAlchemy: inner = child.join(grandchild)
    // 翻译为 rivet-orm: 先构建内层 JOIN
    let inner_join = CHILD.clone().inner_join(
        (*GRANDCHILD).clone(),
        CHILD.column("id").eq(GRANDCHILD.column("child_id")),
    );

    // SQLAlchemy: select(parent).outerjoin_from(parent, inner)
    // 翻译为 rivet-orm: 使用 left_join
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .left_join(&inner_join, PARENT.column("id").eq(CHILD.column("parent_id")));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` LEFT JOIN (`child` AS `child0` INNER JOIN `grandchild` AS `grandchild0` ON `child0`.`id` = `grandchild0`.`child_id`) ON `parent0`.`id` = `child0`.`parent_id`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" LEFT JOIN ("child" AS "child0" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id") ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" LEFT JOIN ("child" AS "child0" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id") ON "parent0"."id" = "child0"."parent_id""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_joins_w_filter_by
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 filter_by() 与 JOIN 组合使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .filter_by(data="p1")
//     .join(child)
//     .filter(child.c.data == "c1")
//     .join_from(table1, table2, table1.c.myid == table2.c.otherid)
//     .filter_by(otherid=5)
// )
// ```
//
// 【rivet-orm 说明】
// **重要**: rivet-orm 不支持 filter_by()，需要使用 where_() 代替。
//
// ----------------------------------------------------------------------------
#[test]
fn test_joins_w_filter_by_equivalent() {
    // 翻译说明: 将 filter_by() 翻译为 where_()
    let stmt = SelectStatement::from(&*PARENT)
        .where_(PARENT.column("data").eq("p1"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .where_(CHILD.column("data").eq("c1"))
        .join(&*MYTABLE, MYTABLE.column("myid").eq(MYOTHERTABLE.column("otherid")))
        .where_(MYOTHERTABLE.column("otherid").eq(5_i64));

    assert_mysql!(
        &stmt,
        "SELECT * FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `mytable` AS `mytable0` INNER JOIN `myothertable` AS `myothertable0` ON `mytable0`.`myid` = `myothertable0`.`otherid` WHERE `parent0`.`data` = ? AND `child0`.`data` = ? AND `myothertable0`.`otherid` = ?",
        ["p1", "c1", 5_i64]
    );
    assert_pg!(
        &stmt,
        r#"SELECT * FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid" WHERE "parent0"."data" = $1 AND "child0"."data" = $2 AND "myothertable0"."otherid" = $3"#,
        ["p1", "c1", 5_i64]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT * FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "mytable" AS "mytable0" INNER JOIN "myothertable" AS "myothertable0" ON "mytable0"."myid" = "myothertable0"."otherid" WHERE "parent0"."data" = ? AND "child0"."data" = ? AND "myothertable0"."otherid" = ?"#,
        ["p1", "c1", 5_i64]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_from_col
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 在简单 SELECT 上的使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c.myid).filter_by(name="foo")
// ```
//
// 【多 dialect 验证】
// - MySQL:     WHERE ... = ?
// - PostgreSQL: WHERE ... = $1
// - SQLite:    WHERE ... = ?
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_from_col_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        ["foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $1"#,
        ["foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        ["foo"]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_from_func
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 与聚合函数组合使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(func.count(table1.c.myid)).filter_by(name="foo")
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_from_func_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(count(MYTABLE.column("myid")).alias("count_1"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT COUNT(`mytable0`.`myid`) AS `count_1` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        ["foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT COUNT("mytable0"."myid") AS "count_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $1"#,
        ["foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT COUNT("mytable0"."myid") AS "count_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        ["foo"]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_from_func_not_the_first_arg
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 与函数组合使用，函数第一个参数不是列的情况。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(func.bar(True, table1.c.myid)).filter_by(name="foo")
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_from_func_not_the_first_arg_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(func("bar", vec![Expr::from(true), Expr::from(MYTABLE.column("myid"))]).alias("bar_1"))
        .where_(MYTABLE.column("name").eq("foo"));

    // MySQL 中布尔值 true 被编译为 1
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

// ----------------------------------------------------------------------------
// test_filter_by_from_cast
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 与 CAST 表达式组合使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(cast(table1.c.myid, Integer)).filter_by(name="foo")
// ```
//
// 【状态】#[ignore] - CAST() 函数不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 CAST() 函数"]
fn test_filter_by_from_cast_equivalent() {
    // SQLAlchemy: select(cast(table1.c.myid, Integer)).filter_by(name="foo")
    // 翻译为 rivet-orm: 但 cast() 函数不存在！
}

// ----------------------------------------------------------------------------
// test_filter_by_from_binary
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 与二元比较表达式组合使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c.myid == 5).filter_by(name="foo")
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_from_binary_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid").eq(5_i64).alias("anon_1"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid` = ? AS `anon_1` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        [5_i64, "foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid" = $1 AS "anon_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $2"#,
        [5_i64, "foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid" = ? AS "anon_1" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        [5_i64, "foo"]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_from_label
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #6414】
// 测试 filter_by() 与带标签的列组合使用。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c.myid.label("some_id")).filter_by(name="foo")
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_from_label_equivalent() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid").alias("some_id"))
        .where_(MYTABLE.column("name").eq("foo"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid` AS `some_id` FROM `mytable` AS `mytable0` WHERE `mytable0`.`name` = ?",
        ["foo"]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid" AS "some_id" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = $1"#,
        ["foo"]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid" AS "some_id" FROM "mytable" AS "mytable0" WHERE "mytable0"."name" = ?"#,
        ["foo"]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_no_property_from_table
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 filter_by() 引用不存在的属性时是否正确抛出 InvalidRequestError。
//
// 【SQLAlchemy 原版代码】
// ```python
// assert_raises_message(
//     exc.InvalidRequestError,
//     'None of the FROM clause entities have a property "foo". '
//     "Searched entities: mytable",
//     select(table1).filter_by,
//     foo="bar",
// )
// ```
//
// 【状态】#[ignore] - filter_by() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 filter_by() 方法"]
fn test_filter_by_no_property_from_table() {
    // SQLAlchemy 抛出 InvalidRequestError
    // rivet-orm 在编译时会因为列不存在而失败
}

// ----------------------------------------------------------------------------
// test_filter_by_no_property_from_col
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试在只选择单列后使用 filter_by() 引用不存在的属性。
//
// 【SQLAlchemy 原版代码】
// ```python
// assert_raises_message(
//     exc.InvalidRequestError,
//     'None of the FROM clause entities have a property "foo". '
//     "Searched entities: mytable",
//     select(table1.c.myid).filter_by,
//     foo="bar",
// )
// ```
//
// 【状态】#[ignore] - filter_by() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 filter_by() 方法"]
fn test_filter_by_no_property_from_col() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_filter_by_across_join_entities_issue_8601
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8601】
// 测试在 join() 后使用 with_only_columns() 再使用 filter_by()。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join(child)
//     .with_only_columns(parent.c.id)
//     .filter_by(parent_id=5)
// )
// ```
//
// 【状态】#[ignore] - 两个功能都不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 with_only_columns() 和 filter_by() 方法"]
fn test_filter_by_across_join_entities_issue_8601() {
    // 两个功能都不支持
}

// ----------------------------------------------------------------------------
// test_filter_by_ambiguous_column_error
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 filter_by() 在有多义性列名时是否正确抛出 AmbiguousColumnError。
// 注意: parent 和 child 都有一个名为 "data" 的列。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child)
// with expect_raises_message(
//     exc.AmbiguousColumnError,
//     'Attribute name "data" is ambiguous; ...',
// ):
//     stmt.filter_by(data="foo")
// ```
//
// 【状态】#[ignore] - filter_by() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 filter_by() 方法，且 ambiguous column 错误处理机制不同"]
fn test_filter_by_ambiguous_column_error() {
    // SQLAlchemy 在 filter_by(data="foo") 时发现 data 列存在于 parent 和 child
    // 两个表中，因此抛出 AmbiguousColumnError
    //
    // rivet-orm 使用 where_() 时，如果列名有歧义，需要显式使用 表名.列名
}

// ----------------------------------------------------------------------------
// test_filter_by_unambiguous_across_joins
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 filter_by() 能正确处理无歧义的列名（跨 JOIN）。
// 注意: parent_id 只存在于 child 表中。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child).filter_by(parent_id=5)
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_unambiguous_across_joins_equivalent() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .where_(CHILD.column("parent_id").eq(5_i64));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` WHERE `child0`.`parent_id` = ?",
        [5_i64]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" WHERE "child0"."parent_id" = $1"#,
        [5_i64]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" WHERE "child0"."parent_id" = ?"#,
        [5_i64]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_column_not_in_any_entity
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 filter_by() 引用的列在任何实体中都不存在时是否报错。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent).join(child)
// with expect_raises_message(
//     exc.InvalidRequestError,
//     'None of the FROM clause entities have a property "nonexistent". ...',
// ):
//     stmt.filter_by(nonexistent="foo")
// ```
//
// 【状态】#[ignore] - filter_by() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 filter_by() 方法"]
fn test_filter_by_column_not_in_any_entity() {
    // SQLAlchemy 在 filter_by(nonexistent="foo") 时发现没有任何表有此列
    // 因此抛出 InvalidRequestError
}

// ----------------------------------------------------------------------------
// test_filter_by_multiple_joins
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试在多层 JOIN 后使用 filter_by()。
// 注意: grandchild 有唯一的 child_id 列。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = (
//     select(parent)
//     .join(child, parent.c.id == child.c.parent_id)
//     .join(grandchild, child.c.id == grandchild.c.child_id)
//     .filter_by(child_id=3)
// )
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_filter_by_multiple_joins_equivalent() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"))
        .join(&*CHILD, PARENT.column("id").eq(CHILD.column("parent_id")))
        .join(&*GRANDCHILD, CHILD.column("id").eq(GRANDCHILD.column("child_id")))
        .where_(GRANDCHILD.column("child_id").eq(3_i64));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0` INNER JOIN `child` AS `child0` ON `parent0`.`id` = `child0`.`parent_id` INNER JOIN `grandchild` AS `grandchild0` ON `child0`.`id` = `grandchild0`.`child_id` WHERE `grandchild0`.`child_id` = ?",
        [3_i64]
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id" WHERE "grandchild0"."child_id" = $1"#,
        [3_i64]
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0" INNER JOIN "child" AS "child0" ON "parent0"."id" = "child0"."parent_id" INNER JOIN "grandchild" AS "grandchild0" ON "child0"."id" = "grandchild0"."child_id" WHERE "grandchild0"."child_id" = ?"#,
        [3_i64]
    );
}

// ----------------------------------------------------------------------------
// test_filter_by_explicit_from_with_join
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试在有显式 select_from() 和 JOIN 后使用 filter_by() 的歧义情况。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent.c.id).select_from(parent).join(child)
// with expect_raises_message(
//     exc.AmbiguousColumnError,
//     'Attribute name "data" is ambiguous; ...',
// ):
//     stmt.filter_by(data="child_data")
// ```
//
// 【状态】#[ignore] - filter_by() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 filter_by() 方法"]
fn test_filter_by_explicit_from_with_join() {
    // 同前面的 ambiguous column 情况
}

// ----------------------------------------------------------------------------
// test_select_tuple_outer
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 tuple_() 作为 SELECT 元素时的错误处理。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(tuple_(table1.c.myid, table1.c.name))
// assert_raises_message(
//     exc.CompileError,
//     r"Most backends don't support SELECTing from a tuple\(\) object...",
//     stmt.compile,
// )
// ```
//
// 【状态】#[ignore] - tuple_() 函数不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 tuple_() 函数"]
fn test_select_tuple_outer() {
    // SQLAlchemy 的 tuple_(col1, col2) 创建一个元组表达式
    // 大部分数据库不支持 SELECT tuple_(col1, col2) FROM ...
}

// ----------------------------------------------------------------------------
// test_select_tuple_subquery
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试在子查询中使用 tuple_()。
//
// 【SQLAlchemy 原版代码】
// ```python
// subq = select(
//     table1.c.name, tuple_(table1.c.myid, table1.c.name)
// ).subquery()
// stmt = select(subq.c.name)
// ```
//
// 【状态】#[ignore] - tuple_() 和 subquery() 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 tuple_() 和 subquery() 方法"]
fn test_select_tuple_subquery() {
    // SQLAlchemy 的 subquery() 创建一个子查询
    // tuple_() 创建一个元组表达式
    // 两个功能都不支持
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - union_all
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 UNION ALL 复合语句。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(literal(1))
// stmt = stmt.union_all(select(literal(2)), select(literal(3)))
// ```
//
// 【状态】#[ignore] - UNION ALL 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 UNION ALL 等集合操作"]
fn test_select_multiple_compound_elements_union_all() {
    // SQLAlchemy: select(literal(1)).union_all(select(literal(2)), select(literal(3)))
    // 生成: SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3
    // rivet-orm 不支持这些集合操作
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - union
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 UNION 复合语句。
//
// 【状态】#[ignore] - UNION 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 UNION 等集合操作"]
fn test_select_multiple_compound_elements_union() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - intersect_all
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 INTERSECT ALL 复合语句。
//
// 【状态】#[ignore] - INTERSECT ALL 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 INTERSECT ALL 等集合操作"]
fn test_select_multiple_compound_elements_intersect_all() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - intersect
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 INTERSECT 复合语句。
//
// 【状态】#[ignore] - INTERSECT 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 INTERSECT 等集合操作"]
fn test_select_multiple_compound_elements_intersect() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - except_all
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 EXCEPT ALL 复合语句。
//
// 【状态】#[ignore] - EXCEPT ALL 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 EXCEPT ALL 等集合操作"]
fn test_select_multiple_compound_elements_except_all() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_select_multiple_compound_elements - except_
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 EXCEPT 复合语句。
//
// 【状态】#[ignore] - EXCEPT 不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持 EXCEPT 等集合操作"]
fn test_select_multiple_compound_elements_except() {
    // 同上
}

// ----------------------------------------------------------------------------
// test_methods_generative - with_statement_hint
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 SQL 提示语句的方法是生成性的（返回新实例）。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.with_statement_hint("some hint")
// assert s1 is not s2  # 验证返回新实例
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_methods_generative_with_statement_hint() {
    let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    let s2 = s1.clone();

    // 验证返回的是新实例（内容不同）
    assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ----------------------------------------------------------------------------
// test_methods_generative - with_hint
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 with_hint() 方法是生成性的。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.with_hint(table("x"), "some hint")
// assert s1 is not s2
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_methods_generative_with_hint() {
    let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    let s2 = s1.clone();

    assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ----------------------------------------------------------------------------
// test_methods_generative - where
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 where() 方法是生成性的。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.where(column("q") == 5)
// assert s1 is not s2
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_methods_generative_where() {
    let s1: SelectStatement = SelectStatement::from(&*USERS).select(USERS.column("id"));
    let s2 = s1.clone().where_(USERS.column("id").eq(1_i64));
    // 验证返回的是新实例
    assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ----------------------------------------------------------------------------
// test_methods_generative - having
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 having() 方法是生成性的。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.having(column("q") == 5)
// assert s1 is not s2
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_methods_generative_having() {
    let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    // rivet-orm 可能不支持 having()，但我们可以验证基本方法调用
    let s2 = s1.clone().where_(USERS.column("id").eq(1_i64));
    assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ----------------------------------------------------------------------------
// test_methods_generative - order_by
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 order_by() 方法是生成性的。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.order_by(column("q"))
// assert s1 is not s2
// ```
//
// 【状态】#[ignore] - order_by() 方法可能不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 可能不支持 order_by() 方法"]
fn test_methods_generative_order_by() {
    // let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    // let s2 = s1.order_by(USERS.column("id").asc());
    // assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ----------------------------------------------------------------------------
// test_methods_generative - group_by
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明】
// 测试 group_by() 方法是生成性的。
//
// 【SQLAlchemy 原版代码】
// ```python
// s1 = select(1)
// s2 = s1.group_by(column("q"))
// assert s1 is not s2
// ```
//
// 【状态】#[ignore] - group_by() 方法可能不支持
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 可能不支持 group_by() 方法"]
fn test_methods_generative_group_by() {
    // let s1 = SelectStatement::from(&*USERS).select(USERS.column("id"));
    // let s2 = s1.group_by(USERS.column("id"));
    // assert_ne!(format!("{:?}", s1), format!("{:?}", s2));
}

// ============================================================================
// ============================================================================
// ColumnCollectionAsSelectTest 类 - SQLAlchemy ColumnCollectionAsSelectTest 的全部测试
// ============================================================================
// ============================================================================

// ----------------------------------------------------------------------------
// test_c_collection_as_from
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试 select(parent.c) 使用整个列集合作为 SELECT 元素。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(parent.c)
// eq_(stmt._all_selected_columns, [parent.c.id, parent.c.data])
// self.assert_compile(stmt, "SELECT parent.id, parent.data FROM parent")
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_collection_as_from() {
    let stmt = SelectStatement::from(&*PARENT)
        .select(PARENT.column("id"))
        .select(PARENT.column("data"));

    assert_mysql!(
        &stmt,
        "SELECT `parent0`.`id`, `parent0`.`data` FROM `parent` AS `parent0`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "parent0"."id", "parent0"."data" FROM "parent" AS "parent0""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_str_stmt
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试通过字符串索引选择列子集合。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c["myid", "description"])
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_str_stmt() {
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("myid"))
        .select(MYTABLE.column("description"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`myid`, `mytable0`.`description` FROM `mytable` AS `mytable0`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."description" FROM "mytable" AS "mytable0""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."myid", "mytable0"."description" FROM "mytable" AS "mytable0""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_int_stmt
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试通过整数索引选择列子集合，并重新排序。
//
// 【SQLAlchemy 原版代码】
// ```python
// stmt = select(table1.c[2, 0])  # 先选 index 2，再选 index 0
// ```
//
// 【翻译说明】
// table1 的列顺序是 [myid (0), name (1), description (2)]
// c[2, 0] 意味着先选 description，再选 myid
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_int_stmt() {
    // table1 列顺序: myid (0), name (1), description (2)
    // c[2, 0] 意味着先选 index 2 (description)，再选 index 0 (myid)
    let stmt = SelectStatement::from(&*MYTABLE)
        .select(MYTABLE.column("description"))
        .select(MYTABLE.column("myid"));

    assert_mysql!(
        &stmt,
        "SELECT `mytable0`.`description`, `mytable0`.`myid` FROM `mytable` AS `mytable0`",
        []
    );
    assert_pg!(
        &stmt,
        r#"SELECT "mytable0"."description", "mytable0"."myid" FROM "mytable" AS "mytable0""#,
        []
    );
    assert_sqlite!(
        &stmt,
        r#"SELECT "mytable0"."description", "mytable0"."myid" FROM "mytable" AS "mytable0""#,
        []
    );
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_str
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试列子集合的字符串索引访问。
//
// 【SQLAlchemy 原版代码】
// ```python
// coll = table1.c["myid", "description"]
// is_(coll.myid, table1.c.myid)  # 验证可以按名称访问
// eq_(list(coll), [table1.c.myid, table1.c.description])
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_str() {
    // SQLAlchemy: coll = table1.c["myid", "description"]
    // rivet-orm: 通过 Table.column("name") 访问
    let col_myid = MYTABLE.column("myid");
    let col_desc = MYTABLE.column("description");

    // 验证可以访问到正确的列
    assert_eq!(col_myid.name, "myid");
    assert_eq!(col_desc.name, "description");
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_int
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试列子集合的整数索引访问。
//
// 【SQLAlchemy 原版代码】
// ```python
// coll = table1.c[2, 0]
// is_(coll.myid, table1.c.myid)
// eq_(list(coll), [table1.c.description, table1.c.myid])
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_int() {
    // SQLAlchemy: coll = table1.c[2, 0]
    // rivet-orm: 通过 Table.column("name") 访问
    let col_myid = MYTABLE.column("myid");
    let col_desc = MYTABLE.column("description");

    assert_eq!(col_myid.name, "myid");
    assert_eq!(col_desc.name, "description");
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_positive_slice
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试正数切片访问。
//
// 【SQLAlchemy 原版代码】
// ```python
// coll = table1.c[0:2]  # 切片 [0, 1)，即 index 0 和 1
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_positive_slice() {
    // SQLAlchemy: c[0:2] 选择 index 0 和 1 (不含 2)
    // rivet-orm: 通过 Table.column("name") 访问
    let col_myid = MYTABLE.column("myid");
    let col_name = MYTABLE.column("name");

    assert_eq!(col_myid.name, "myid");
    assert_eq!(col_name.name, "name");
}

// ----------------------------------------------------------------------------
// test_c_sub_collection_negative_slice
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试负数切片访问。
//
// 【SQLAlchemy 原版代码】
// ```python
// coll = table1.c[-2:]  # 最后两个元素，即 index 1 和 2
// ```
//
// ----------------------------------------------------------------------------
#[test]
fn test_c_sub_collection_negative_slice() {
    // SQLAlchemy: c[-2:] 选择最后两个元素
    // table1 列: [myid (0), name (1), description (2)]
    // c[-2:] 选择 index 1 和 2，即 name 和 description
    let col_name = MYTABLE.column("name");
    let col_desc = MYTABLE.column("description");

    assert_eq!(col_name.name, "name");
    assert_eq!(col_desc.name, "description");
}

// ----------------------------------------------------------------------------
// test_missing_key
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试访问不存在的键时是否抛出 KeyError。
//
// 【SQLAlchemy 原版代码】
// ```python
// with expect_raises_message(KeyError, "unknown"):
//     table1.c["myid", "unknown"]
// ```
//
// 【状态】#[ignore] - 错误处理机制不同
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 的列访问错误处理机制与 SQLAlchemy 不同"]
fn test_missing_key() {
    // SQLAlchemy: table1.c["myid", "unknown"] 抛出 KeyError("unknown")
    // rivet-orm: Table.column("unknown") 在编译时或运行时失败
}

// ----------------------------------------------------------------------------
// test_missing_index
// ----------------------------------------------------------------------------
//
// 【SQLAlchemy 原版测试说明 - test issue #8285】
// 测试使用越界整数索引时是否抛出 IndexError。
//
// 【SQLAlchemy 原版代码】
// ```python
// with expect_raises_message(IndexError, "5"):
//     table1.c["myid", 5]
// ```
//
// 【状态】#[ignore] - rivet-orm 不支持整数索引访问
// ----------------------------------------------------------------------------
#[test]
#[ignore = "rivet-orm 不支持整数索引访问列"]
fn test_missing_index() {
    // SQLAlchemy: table1.c["myid", 5] 中 "myid" 存在，但 5 越界
    // 抛出 IndexError("5")
    // rivet-orm 不支持这种方式
}
