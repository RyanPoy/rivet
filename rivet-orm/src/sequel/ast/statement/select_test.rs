use crate::sequel::ast::{Column, Operand, SelectStatement, Source, Table};

mod binder {
    use crate::sequel::build::{Binder, Dialect};

    pub fn mysql() -> Binder {
        Binder::new(Dialect::MySql)
    }
    pub fn pg() -> Binder {
        Binder::new(Dialect::PostgreSql)
    }
    pub fn sqlite() -> Binder {
        Binder::new(Dialect::Sqlite)
    }
}

#[test]
fn test_empty_query() {
    let stmt = SelectStatement::new().from(Source::Table(Table::new("abc")));

    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT * FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT * FROM "abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT * FROM "abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_table_schema() {
    let stmt = SelectStatement::new().from(Source::Table(Table::new("abc").schema("schema1")));
    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT * FROM `schema1`.`abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT * FROM "schema1"."abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT * FROM "schema1"."abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_distinct_single() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc")))
        .select(Operand::Column(Column::new("foo")))
        .distinct();
    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT DISTINCT `foo` FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT DISTINCT "foo" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT DISTINCT "foo" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_distinct_multi() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc")))
        .select(Operand::Column(Column::new("foo")))
        .select(Operand::Column(Column::new("bar")))
        .distinct();
    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT DISTINCT `foo`, `bar` FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT DISTINCT "foo", "bar" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT DISTINCT "foo", "bar" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_single_column() {
    let stmt =
        SelectStatement::new().from(Source::Table(Table::new("abc"))).select(Operand::Column(Column::new("foo")));
    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT `foo` FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT "foo" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT "foo" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_single_column_with_alias() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc")))
        .select(Operand::Column(Column::new("foo").alias("bar")));
    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT `foo` AS `bar` FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, "SELECT \"foo\" AS \"bar\" FROM \"abc\"".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, "SELECT \"foo\" AS \"bar\" FROM \"abc\"".to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_single_column_and_table_alias_str() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc").alias("fizzbuzz")))
        .select(Operand::Column(Column::new("foo").alias("bar")));

    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT `foo` AS `bar` FROM `abc` AS `fizzbuzz`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT "foo" AS "bar" FROM "abc" AS "fizzbuzz""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT "foo" AS "bar" FROM "abc" AS "fizzbuzz""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_multiple_columns() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc")))
        .select(Operand::Column(Column::new("foo")))
        .select(Operand::Column(Column::new("bar")));

    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT `foo`, `bar` FROM `abc`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT "foo", "bar" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT "foo", "bar" FROM "abc""#.to_string());
    assert_eq!(params, vec![]);
}

#[test]
fn test_select_multiple_tables() {
    let stmt = SelectStatement::new()
        .from(Source::Table(Table::new("abc")))
        .select(Operand::Column(Column::new("foo").schema("abc")))
        .from(Source::Table(Table::new("efg")))
        .select(Operand::Column(Column::new("bar").schema("efg")));

    let (sql, params) = stmt.to_sql(&mut binder::mysql());
    assert_eq!(sql, "SELECT `abc`.`foo`, `efg`.`bar` FROM `abc`, `efg`".to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::sqlite());
    assert_eq!(sql, r#"SELECT "abc"."foo", "efg"."bar" FROM "abc", "efg""#.to_string());
    assert_eq!(params, vec![]);

    let (sql, params) = stmt.to_sql(&mut binder::pg());
    assert_eq!(sql, r#"SELECT "abc"."foo", "efg"."bar" FROM "abc", "efg""#.to_string());
    assert_eq!(params, vec![]);
}

// #[test]
// fn test_select_subquery(){
// #     sub = SelectStatement().from_(Name("abc"))
// #     stmt = SelectStatement().from_(sub).select(Name("foo"), Name("bar"))
// # visitors.mysql.sql(stmt) ==     self.assertEqual('SELECT "sq0"."foo", "sq0"."bar" FROM (SELECT * FROM "abc") AS "sq0"'
// # visitors.sqlite.sql(stmt) ==     self.assertEqual('SELECT "sq0"."foo", "sq0"."bar" FROM (SELECT * FROM "abc") AS "sq0"'
// # visitors.pg.sql(stmt) ==     self.assertEqual('SELECT "sq0"."foo", "sq0"."bar" FROM (SELECT * FROM "abc") AS "sq0"'
//
// #     }
//
// #[test]
// fn test_select__multiple_subqueries(){
// #         subquery0 = SelectStatement().from_(Name("abc")).select(Name("foo"))
// #         subquery1 = SelectStatement().from_(Name("efg")).select("bar")
// #         stmt = SelectStatement().from_(subquery0).from_(subquery1).select(subquery0.foo, subquery1.bar)
// #
// #       assert
// #             'SELECT "sq0"."foo","sq1"."bar" ' 'FROM (SELECT "foo" FROM "abc") "sq0",' '(SELECT "bar" FROM "efg") "sq1"',
// #             str(q),
// #         )
// #
// #     }
//
// #[test]
// fn test_select__nested_subquery(){
// #         subquery0 = SelectStatement().from_(Name("abc"))
// #         subquery1 = SelectStatement().from_(subquery0).select(subquery0.foo, subquery0.bar)
// #         subquery2 = SelectStatement().from_(subquery1).select(subquery1.foo)
// #
// #         stmt = SelectStatement().from_(subquery2).select(subquery2.foo)
// #
// #       assert
// #             'SELECT "sq2"."foo" '
// #             'FROM (SELECT "sq1"."foo" '
// #             'FROM (SELECT "sq0"."foo","sq0"."bar" '
// #             'FROM (SELECT * FROM "abc") "sq0") "sq1") "sq2"',
// #             str(q),
// #         )
// #
// }
//
// #[test]
// fn test_select__no_table(){
//     stmt = SelectStatement().select(1, 2, 3)
//     assert visitors.mysql.sql(stmt) == "SELECT 1, 2, 3"
//     assert visitors.sqlite.sql(stmt) == "SELECT 1, 2, 3"
//     assert visitors.pg.sql(stmt) == "SELECT 1, 2, 3"
//
//
// }
//
// #[test]
// fn test_select_then_add_table(){
//     stmt = SelectStatement().select(1, 2, 3).from_(Name("abc")).select("foo").select(Name("bar"))
//     assert visitors.mysql.sql(stmt) == 'SELECT 1, 2, 3, \'foo\', `bar` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT 1, 2, 3, \'foo\', "bar" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT 1, 2, 3, \'foo\', "bar" FROM "abc"'
//
//
// }
//
// #[test]
// fn test_select_with_limit(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).limit(10)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` LIMIT 10'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" LIMIT 10'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" LIMIT 10'
//
//
// }
//
// #[test]
// fn test_select_with_limit_zero(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).limit(0)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc"'
//
//
// }
//
// #[test]
// fn test_select_with_offset(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).offset(10)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` OFFSET 10'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" OFFSET 10'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" OFFSET 10'
//
//
// }
//
// #[test]
// fn test_select_with_limit_and_offset(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).offset(10).limit(10)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` LIMIT 10 OFFSET 10'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" LIMIT 10 OFFSET 10'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" LIMIT 10 OFFSET 10'
//
//
// }
//
// #[test]
// fn test_select_with_force_index(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).force_index(Name("egg"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` FORCE INDEX (`egg`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg")'
//
//
// }
//
// #[test]
// fn test_select_with_force_index_multiple_indexes(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).force_index(Name("egg"), Name("bacon"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` FORCE INDEX (`egg`, `bacon`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg", "bacon")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg", "bacon")'
//
//
// }
//
// #[test]
// fn test_select_with_force_index_multiple_calls(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).force_index(Name("egg")).force_index(Name("spam"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` FORCE INDEX (`egg`, `spam`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg", "spam")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg", "spam")'
//
//
// }
//
// #[test]
// fn test_select_with_use_index(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).use_index(Name("egg"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` USE INDEX (`egg`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg")'
//
//
// }
//
// #[test]
// fn test_select_with_use_index_multiple_indexes(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).use_index(Name("egg"), Name("bacon"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` USE INDEX (`egg`, `bacon`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg", "bacon")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg", "bacon")'
//
//
// }
//
// #[test]
// fn test_select_with_use_index_multiple_calls(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).use_index(Name("egg")).use_index(Name("spam"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` USE INDEX (`egg`, `spam`)'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg", "spam")'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" USE INDEX ("egg", "spam")'
//
//
// }
//
// #[test]
// fn test_table_select_alias(){
//     stmt = SelectStatement().from_(Name("abc")).select(1)
//     assert visitors.mysql.sql(stmt) == 'SELECT 1 FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT 1 FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT 1 FROM "abc"'
//
//
// }
//
// #[test]
// fn test_where_basic(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo="foo")
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` = 'foo'"
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'foo\''
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'foo\''
//
//     stmt = SelectStatement().from_(Name("abc")).where(foo=0)
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` = 0"
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = 0'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = 0'
//
//     stmt = SelectStatement().from_(Name("abc")).where(foo=True)
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` = 1"
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = 1'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = 1'
//
//     stmt = SelectStatement().from_(Name("abc")).where(foo=date(2020, 2, 2))
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` = '2020-02-02'"
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\''
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\''
//
//     stmt = SelectStatement().from_(Name("abc")).where(foo=None)
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` IS NULL"
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" IS NULL'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" IS NULL'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo=date(2020, 2, 2)).for_update()
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'2020-02-02\' FOR UPDATE'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update_share(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo=date(2020, 2, 2)).for_update(share=True)
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'2020-02-02\' FOR UPDATE SHARE'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE SHARE'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE SHARE'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update_nowait(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo=date(2020, 2, 2)).for_update(nowait=True)
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'2020-02-02\' FOR UPDATE NOWAIT'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE NOWAIT'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE NOWAIT'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update_skip(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo=date(2020, 2, 2)).for_update(skip=True)
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'2020-02-02\' FOR UPDATE SKIP LOCKED'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE SKIP LOCKED'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'2020-02-02\' FOR UPDATE SKIP LOCKED'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update_of(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo="bar").for_update(of=("abc",))
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'bar\' FOR UPDATE OF `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'bar\' FOR UPDATE OF "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'bar\' FOR UPDATE OF "abc"'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_update_skip_locked_and_of(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo="bar").for_update(skip=True, of=("abc",))
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `foo` = \'bar\' FOR UPDATE OF `abc` SKIP LOCKED'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'bar\' FOR UPDATE OF "abc" SKIP LOCKED'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" = \'bar\' FOR UPDATE OF "abc" SKIP LOCKED'
//
//
// }
//
// #[test]
// fn test_where_field_equals_for_multiple_tables(){
//     stmt = (SelectStatement().from_(Name("abc"))
//             .join(Name("efg")).on(abc__id=Name("id", "efg"))
//             .where(abc__foo=Name("bar", "efg"))
//             )
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` JOIN `efg` ON `abc`.`id` = `efg`.`id` WHERE `abc`.`foo` = `efg`.`bar`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" JOIN "efg" ON "abc"."id" = "efg"."id" WHERE "abc"."foo" = "efg"."bar"'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" JOIN "efg" ON "abc"."id" = "efg"."id" WHERE "abc"."foo" = "efg"."bar"'
//
//
// }
//
// #[test]
// fn test_where_field_equals_where(){
//     stmt = SelectStatement().from_(Name("abc")).where(abc__foo=1, abc__bar=Name('baz', Name("abc").name))
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE `abc`.`foo` = 1 AND `abc`.`bar` = `abc`.`baz`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE "abc"."foo" = 1 AND "abc"."bar" = "abc"."baz"'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE "abc"."foo" = 1 AND "abc"."bar" = "abc"."baz"'
//
//
// }
//
// #[test]
// fn test_where_field_equals_where_not(){
//     stmt = SelectStatement().from_(Name("abc")).where(~Binary.parse(foo=1)).where(bar=Name('baz', schema_name=Name("abc").name))
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `abc` WHERE NOT `foo` = 1 AND `bar` = `abc`.`baz`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "abc" WHERE NOT "foo" = 1 AND "bar" = "abc"."baz"'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "abc" WHERE NOT "foo" = 1 AND "bar" = "abc"."baz"'
//
//
// }
//
// #[test]
// fn test_where_single_quote(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo="bar'foo")
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` = 'bar''foo'"
//     assert visitors.sqlite.sql(stmt) == "SELECT * FROM \"abc\" WHERE \"foo\" = 'bar''foo'"
//     assert visitors.pg.sql(stmt) == "SELECT * FROM \"abc\" WHERE \"foo\" = 'bar''foo'"
//
//
// }
//
// #[test]
// fn test_where_field_matches_regex(){
//     stmt = SelectStatement().from_(Name("abc")).where(foo__regex="r^b")
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc` WHERE `foo` REGEX 'r^b'"
//     assert visitors.sqlite.sql(stmt) == "SELECT * FROM \"abc\" WHERE \"foo\" REGEX 'r^b'"
//     assert visitors.pg.sql(stmt) == "SELECT * FROM \"abc\" WHERE \"foo\" REGEX 'r^b'"
//
//
// }
//
// #[test]
// fn test_ignore_empty_criterion_where(){
//     stmt = SelectStatement().from_(Name("abc")).where()
//     assert visitors.mysql.sql(stmt) == "SELECT * FROM `abc`"
//     assert visitors.sqlite.sql(stmt) == "SELECT * FROM \"abc\""
//     assert visitors.pg.sql(stmt) == "SELECT * FROM \"abc\""
//
//
// }
//
// #[test]
// fn test_select_with_force_index_and_where(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo")).where(foo="bar").force_index(Name("egg"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` FORCE INDEX (`egg`) WHERE `foo` = \'bar\''
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg") WHERE "foo" = \'bar\''
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" FORCE INDEX ("egg") WHERE "foo" = \'bar\''
//
//
// }
//
// #[test]
// fn test_group_by__single(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo).select(foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_group_by__multi(){
//     foo, bar = Name("foo"), Name("bar")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo, bar).select(foo, bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, `bar` FROM `abc` GROUP BY `foo`, `bar`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", "bar" FROM "abc" GROUP BY "foo", "bar"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", "bar" FROM "abc" GROUP BY "foo", "bar"'
//
//
// }
//
// #[test]
// fn test_group_by__count_star(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo).select(foo, Count(STAR))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, COUNT(*) FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", COUNT(*) FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", COUNT(*) FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_group_by__count_field(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo).select(foo, Count(Name("bar")))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, COUNT(`bar`) FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", COUNT("bar") FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", COUNT("bar") FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_group_by__count_distinct(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo).select(foo, Count(STAR).distinct())
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, COUNT(DISTINCT *) FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", COUNT(DISTINCT *) FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", COUNT(DISTINCT *) FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_group_by__sum_distinct(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).group_by(foo).select(foo, Sum(Name("bar")).distinct())
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, SUM(DISTINCT `bar`) FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", SUM(DISTINCT "bar") FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", SUM(DISTINCT "bar") FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_group_by__alias(){
//     bar = Name("bar").as_("bar01")
//     stmt = SelectStatement().from_(Name("abc")).select(Sum(Name("foo")), bar).group_by(bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT SUM(`foo`), `bar` AS `bar01` FROM `abc` GROUP BY `bar01`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT SUM("foo"), "bar" AS "bar01" FROM "abc" GROUP BY "bar01"'
//     assert visitors.pg.sql(stmt) == 'SELECT SUM("foo"), "bar" AS "bar01" FROM "abc" GROUP BY "bar01"'
//
//
// }
//
// #[test]
// fn test_group_by__alias_with_join(){
//     table1 = Name("table1").as_("t1")
//     bar = Name("bar", schema_name=table1.alias).as_("bar01")
//     stmt = (SelectStatement().from_(Name("abc")).join(table1)
//             .on(abc__id=Name("t_ref", schema_name=table1.alias))
//             .select(Sum(Name("foo")), bar).group_by(bar))
//     assert visitors.mysql.sql(stmt) == 'SELECT SUM(`foo`), `t1`.`bar` AS `bar01` FROM `abc` JOIN `table1` AS `t1` ON `abc`.`id` = `t1`.`t_ref` GROUP BY `bar01`'
//     assert visitors.sqlite.sql(
//         stmt) == 'SELECT SUM("foo"), "t1"."bar" AS "bar01" FROM "abc" JOIN "table1" AS "t1" ON "abc"."id" = "t1"."t_ref" GROUP BY "bar01"'
//     assert visitors.pg.sql(stmt) == 'SELECT SUM("foo"), "t1"."bar" AS "bar01" FROM "abc" JOIN "table1" AS "t1" ON "abc"."id" = "t1"."t_ref" GROUP BY "bar01"'
//
//
// }
//
// #[test]
// fn test_mysql_query_uses_backtick_quote_chars(){
//     stmt = SelectStatement().from_(Name("abc")).group_by(Name('foo')).select(Name('foo'))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` GROUP BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" GROUP BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" GROUP BY "foo"'
//
//
// }
//
// #[test]
// fn test_having_greater_than(){
//     foo, bar = Name('foo'), Name('bar')
//     stmt = SelectStatement().from_(Name("abc")).select(foo, Sum(bar)).group_by(foo).having(Sum(bar).gt(1))
//
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, SUM(`bar`) FROM `abc` GROUP BY `foo` HAVING SUM(`bar`) > 1'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", SUM("bar") FROM "abc" GROUP BY "foo" HAVING SUM("bar") > 1'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", SUM("bar") FROM "abc" GROUP BY "foo" HAVING SUM("bar") > 1'
//
//
// }
//
// #[test]
// fn test_having_and(){
//     foo, bar = Name('foo'), Name('bar')
//     stmt = SelectStatement().from_(Name("abc")).select(foo, Sum(bar)).group_by(foo).having((Sum(bar).gt(1)) & (Sum(bar).lt(100)))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, SUM(`bar`) FROM `abc` GROUP BY `foo` HAVING SUM(`bar`) > 1 AND SUM(`bar`) < 100'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", SUM("bar") FROM "abc" GROUP BY "foo" HAVING SUM("bar") > 1 AND SUM("bar") < 100'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", SUM("bar") FROM "abc" GROUP BY "foo" HAVING SUM("bar") > 1 AND SUM("bar") < 100'
//
//
// }
//
// #[test]
// fn test_having_join_and_equality(){
//     abc_foo = Name('foo', schema_name=Name("abc").name)
//     abc_buz = Name('buz', schema_name=Name("abc").name)
//     efg_foo = Name('foo', schema_name=Name("efg").name)
//     efg_bar = Name('bar', schema_name=Name("efg").name)
//
//     stmt = (
//         SelectStatement().from_(Name("abc")).join(Name("efg"))
//         .on(abc__foo=efg_foo)
//         .select(abc_foo, Sum(efg_bar), abc_buz)
//         .group_by(abc_foo)
//         .having(abc__buz="fiz")
//         .having(Sum(efg_bar).gt(100))
//     )
//
//     assert visitors.mysql.sql(stmt) == ('SELECT `abc`.`foo`, SUM(`efg`.`bar`), `abc`.`buz` FROM `abc` '
//                                         'JOIN `efg` ON `abc`.`foo` = `efg`.`foo` GROUP BY `abc`.`foo` '
//                                         'HAVING `abc`.`buz` = \'fiz\' AND SUM(`efg`.`bar`) > 100')
//     assert visitors.sqlite.sql(stmt) == ('SELECT "abc"."foo", SUM("efg"."bar"), "abc"."buz" FROM "abc" '
//                                          'JOIN "efg" ON "abc"."foo" = "efg"."foo" GROUP BY "abc"."foo" '
//                                          'HAVING "abc"."buz" = \'fiz\' AND SUM("efg"."bar") > 100')
//     assert visitors.pg.sql(stmt) == ('SELECT "abc"."foo", SUM("efg"."bar"), "abc"."buz" FROM "abc" '
//                                      'JOIN "efg" ON "abc"."foo" = "efg"."foo" GROUP BY "abc"."foo" '
//                                      'HAVING "abc"."buz" = \'fiz\' AND SUM("efg"."bar") > 100')
//
//
// }
//
// #[test]
// fn test_order_by__single_field(){
//     stmt = SelectStatement().from_(Name("abc")).order_by(Name("foo")).select(Name("foo"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` ORDER BY `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo"'
//
//
// }
//
// #[test]
// fn test_order_by__multi_fields(){
//     foo, bar = Name("foo"), Name("bar")
//     stmt = SelectStatement().from_(Name("abc")).order_by(foo, bar).select(foo, bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo`, `bar` FROM `abc` ORDER BY `foo`, `bar`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo", "bar" FROM "abc" ORDER BY "foo", "bar"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo", "bar" FROM "abc" ORDER BY "foo", "bar"'
//
//
// }
//
// #[test]
// fn test_order_by_asc(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).order_by(foo, sorted_in=SortedIn.ASC).select(foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` ORDER BY `foo` ASC'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo" ASC'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo" ASC'
//
//
// }
//
// #[test]
// fn test_order_by_desc(){
//     foo = Name("foo")
//     stmt = SelectStatement().from_(Name("abc")).order_by(foo, sorted_in=SortedIn.DESC).select(foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` FROM `abc` ORDER BY `foo` DESC'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo" DESC'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" FROM "abc" ORDER BY "foo" DESC'
//
//
// }
//
// #[test]
// fn test_order_by__alias(){
//     bar = Name("bar").as_("bar01")
//     stmt = SelectStatement().from_(Name("abc")).select(Sum(Name("foo")), bar).order_by(bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT SUM(`foo`), `bar` AS `bar01` FROM `abc` ORDER BY `bar01`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT SUM("foo"), "bar" AS "bar01" FROM "abc" ORDER BY "bar01"'
//     assert visitors.pg.sql(stmt) == 'SELECT SUM("foo"), "bar" AS "bar01" FROM "abc" ORDER BY "bar01"'
//
//
// }
//
// #[test]
// fn test_table_field(){
//     bar = Name("foo").as_("bar")
//     stmt = SelectStatement().from_(Name("abc")).select(bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` AS `bar` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc"'
//
//
// }
//
// #[test]
// fn test_table_field__multi(){
//     stmt = SelectStatement().from_(Name("abc")).select(Name("foo").as_("bar"), Name("fiz").as_("buz"))
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` AS `bar`, `fiz` AS `buz` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" AS "bar", "fiz" AS "buz" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" AS "bar", "fiz" AS "buz" FROM "abc"'
//
//
// # }
//
// #[test]
// fn test_arithmetic_function(){
// #     """ @todo: support arithmetic """
// #     stmt = SelectStatement().from_(Name("abc")).select((self.t.foo + self.t.bar).as_("biz"))
// # visitors.mysql.sql(stmt) ==     self.assertEqual('SELECT "foo"+"bar" "biz" FROM "abc"'
//
// }
//
// #[test]
// fn test_alias_functions(){
//     stmt = SelectStatement().from_(Name("abc")).select(Count(STAR).as_("foo"))
//     assert visitors.mysql.sql(stmt) == 'SELECT COUNT(*) AS `foo` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT COUNT(*) AS "foo" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT COUNT(*) AS "foo" FROM "abc"'
//
//
// }
//
// #[test]
// fn test_alias_function_using_as_nested(){
//     """ We don't show aliases of fields that are arguments of a function. """
//     stmt = SelectStatement().from_(Name("abc")).select(Sqrt(Count(STAR).as_("foo")).as_("bar"))
//     assert visitors.mysql.sql(stmt) == 'SELECT SQRT(COUNT(*)) AS `bar` FROM `abc`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT SQRT(COUNT(*)) AS "bar" FROM "abc"'
//     assert visitors.pg.sql(stmt) == 'SELECT SQRT(COUNT(*)) AS "bar" FROM "abc"'
//
//
// }
//
// #[test]
// fn test_alias_in__group_by(){
//     foo = Name('foo').as_('bar')
//     stmt = SelectStatement().from_(Name("abc")).select(foo).group_by(foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` AS `bar` FROM `abc` GROUP BY `bar`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" GROUP BY "bar"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" GROUP BY "bar"'
//
//
// }
//
// #[test]
// fn test_alias_in__order_by(){
//     foo = Name('foo').as_('bar')
//     stmt = SelectStatement().from_(Name("abc")).select(foo).order_by(foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` AS `bar` FROM `abc` ORDER BY `bar`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" ORDER BY "bar"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" ORDER BY "bar"'
//
//
// }
//
// #[test]
// fn test_alias_ignored__in_value(){
//     foo = Name('foo').as_('bar')
//     stmt = SelectStatement().from_(Name("abc")).select(foo).where(username=foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `foo` AS `bar` FROM `abc` WHERE `username` = `foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" WHERE "username" = "foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "foo" AS "bar" FROM "abc" WHERE "username" = "foo"'
//
//
// }
//
// #[test]
// fn test_select__multiple_tables(){
//     table_abc = Name("abc").as_("t0")
//     table_efg = Name("efg").as_("t1")
//     foo = Name('foo', schema_name=table_abc)
//     bar = Name('bar', schema_name=table_efg)
//     stmt = SelectStatement().from_(table_abc).select(foo).from_(table_efg).select(bar)
//     assert visitors.mysql.sql(stmt) == 'SELECT `t0`.`foo`, `t1`.`bar` FROM `abc` AS `t0`, `efg` AS `t1`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "t0"."foo", "t1"."bar" FROM "abc" AS "t0", "efg" AS "t1"'
//     assert visitors.pg.sql(stmt) == 'SELECT "t0"."foo", "t1"."bar" FROM "abc" AS "t0", "efg" AS "t1"'
//
//
// }
//
// #[test]
// fn test_use_aliases_in__group_by_and_order_by(){
//     table_abc = Name("abc").as_("t0")
//     my_foo = Name("foo", table_abc.alias).as_("my_foo")
//     bar = Name("bar", table_abc.alias)
//     stmt = SelectStatement().from_(table_abc).select(my_foo, bar).group_by(my_foo).order_by(my_foo)
//     assert visitors.mysql.sql(stmt) == 'SELECT `t0`.`foo` AS `my_foo`, `t0`.`bar` FROM `abc` AS `t0` GROUP BY `my_foo` ORDER BY `my_foo`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT "t0"."foo" AS "my_foo", "t0"."bar" FROM "abc" AS "t0" GROUP BY "my_foo" ORDER BY "my_foo"'
//     assert visitors.pg.sql(stmt) == 'SELECT "t0"."foo" AS "my_foo", "t0"."bar" FROM "abc" AS "t0" GROUP BY "my_foo" ORDER BY "my_foo"'
//
//
// }
//
// #[test]
// fn test_table_with_schema_and_alias(){
//     table = Name("abc", schema_name="schema").as_("alias")
//     stmt = SelectStatement().from_(table)
//     assert visitors.mysql.sql(stmt) == 'SELECT * FROM `schema`.`abc` AS `alias`'
//     assert visitors.sqlite.sql(stmt) == 'SELECT * FROM "schema"."abc" AS "alias"'
//     assert visitors.pg.sql(stmt) == 'SELECT * FROM "schema"."abc" AS "alias"'
//
//
// }
//
// #[test]
// fn test_extraneous_quotes(){
//     t1 = Name("table1").as_("t1")
//     t2 = Name("table2").as_("t2")
//     stmt = SelectStatement().from_(t1).join(t2).on(t1__value__bt=(Name("start", schema_name=t2), Name("end", schema_name=t2))).select(
//         Name("value", schema_name=t1))
//     assert visitors.mysql.sql(stmt) == 'SELECT `t1`.`value` FROM `table1` AS `t1` JOIN `table2` AS `t2` ON `t1`.`value` BETWEEN `t2`.`start` AND `t2`.`end`'
//     assert visitors.sqlite.sql(
//         stmt) == 'SELECT "t1"."value" FROM "table1" AS "t1" JOIN "table2" AS "t2" ON "t1"."value" BETWEEN "t2"."start" AND "t2"."end"'
//     assert visitors.pg.sql(stmt) == 'SELECT "t1"."value" FROM "table1" AS "t1" JOIN "table2" AS "t2" ON "t1"."value" BETWEEN "t2"."start" AND "t2"."end"'
//
//     # class SubqueryTests(unittest.TestCase):
//     #     maxDiff = None
//     #
//     #     table_abc, table_efg, table_hij = Tables("abc", "efg", "hij")
//     #
//     #     }
//
// #[test]
// fn test_where__in(){
//     #         stmt = (
//     #             SelectStatement().from_(Name("abc"))
//     #
//     #             .where(
//     #                 Name("abc").foo.isin(
//     #                     SelectStatement().from_(Name("efg")).select(Name("efg").foo).where(Name("efg").bar == 0)
//     #                 )
//     #             )
//     #         )
//     #
//     #       assert
//     #             'SELECT * FROM "abc" WHERE "foo" IN (SELECT "foo" FROM "efg" WHERE "bar"=0)',
//     #             str(q),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_where__in_nested(){
//     #         stmt = SelectStatement().from_(Name("abc")).where(Name("abc").foo).isin(Name("efg"))
//     #         assert visitors.mysql.sql(stmt) == 'SELECT * FROM "abc" WHERE "foo" IN (SELECT * FROM "efg")'
//     #
//     #     }
//
// #[test]
// fn test_join(){
//     #         subquery = SelectStatement().from_("efg").select("fiz", "buz").where(F("buz") == 0)
//     #
//     #         stmt = (
//     #             SelectStatement().from_(Name("abc"))
//     #             .join(subquery)
//     #             .on(Name("abc").bar == subquery.buz)
//     #             .select(Name("abc").foo, subquery.fiz)
//     #         )
//     #
//     #       assert
//     #             'SELECT "abc"."foo","sq0"."fiz" FROM "abc" '
//     #             'JOIN (SELECT "fiz","buz" FROM "efg" WHERE "buz"=0) "sq0" '
//     #             'ON "abc"."bar"="sq0"."buz"',
//     #             str(q),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_select_subquery(){
//     #         substmt = SelectStatement().from_(Name("efg")).select("fizzbuzz").where(Name("efg").id == 1)
//     #
//     #         stmt = SelectStatement().from_(Name("abc")).select("foo", "bar").select(subq)
//     #
//     #       assert
//     #             'SELECT "foo","bar",(SELECT "fizzbuzz" FROM "efg" WHERE "id"=1) ' 'FROM "abc"',
//     #             str(q),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_select_subquery_with_alias(){
//     #         substmt = SelectStatement().from_(Name("efg")).select("fizzbuzz").where(Name("efg").id == 1)
//     #
//     #         stmt = SelectStatement().from_(Name("abc")).select("foo", "bar").select(subq.as_("sq"))
//     #
//     #       assert
//     #             'SELECT "foo","bar",(SELECT "fizzbuzz" FROM "efg" WHERE "id"=1) "sq" ' 'FROM "abc"',
//     #             str(q),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_where__equality(){
//     #         subquery = SelectStatement().from_("efg").select("fiz").where(F("buz") == 0)
//     #         query = (
//     #             SelectStatement().from_(Name("abc"))
//     #             .select(Name("abc").foo, Name("abc").bar)
//     #             .where(Name("abc").bar == subquery)
//     #         )
//     #
//     #       assert
//     #             'SELECT "foo","bar" FROM "abc" ' 'WHERE "bar"=(SELECT "fiz" FROM "efg" WHERE "buz"=0)',
//     #             str(query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_select_from_nested_query(){
//     #         subquery = SelectStatement().from_(Name("abc")).select(
//     #             Name("abc").foo,
//     #             Name("abc").bar,
//     #             (Name("abc").fizz + Name("abc").buzz).as_("fizzbuzz"),
//     #         )
//     #
//     #         query = SelectStatement().from_(subquery).select(subquery.foo, subquery.bar, subquery.fizzbuzz)
//     #
//     #       assert
//     #             'SELECT "sq0"."foo","sq0"."bar","sq0"."fizzbuzz" '
//     #             "FROM ("
//     #             'SELECT "foo","bar","fizz"+"buzz" "fizzbuzz" '
//     #             'FROM "abc"'
//     #             ') "sq0"',
//     #             str(query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_select_from_nested_query_with_join(){
//     #         subquery1 = (
//     #             SelectStatement().from_(Name("abc"))
//     #             .select(
//     #                 Name("abc").foo,
//     #                 fn.Sum(Name("abc").fizz + Name("abc").buzz).as_("fizzbuzz"),
//     #             )
//     #             .group_by(Name("abc").foo)
//     #         )
//     #
//     #         subquery2 = SelectStatement().from_(Name("efg")).select(
//     #             Name("efg").foo.as_("foo_two"),
//     #             Name("efg").bar,
//     #         )
//     #
//     #         query = (
//     #             SelectStatement().from_(subquery1)
//     #             .select(subquery1.foo, subquery1.fizzbuzz)
//     #             .join(subquery2)
//     #             .on(subquery1.foo == subquery2.foo_two)
//     #             .select(subquery2.foo_two, subquery2.bar)
//     #         )
//     #
//     #       assert
//     #             "SELECT "
//     #             '"sq0"."foo","sq0"."fizzbuzz",'
//     #             '"sq1"."foo_two","sq1"."bar" '
//     #             "FROM ("
//     #             "SELECT "
//     #             '"foo",SUM("fizz"+"buzz") "fizzbuzz" '
//     #             'FROM "abc" '
//     #             'GROUP BY "foo"'
//     #             ') "sq0" JOIN ('
//     #             "SELECT "
//     #             '"foo" "foo_two","bar" '
//     #             'FROM "efg"'
//     #             ') "sq1" ON "sq0"."foo"="sq1"."foo_two"',
//     #             str(query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_from_subquery_without_alias(){
//     #         subquery = SelectStatement().from_(Name("efg")).select(
//     #             Name("efg").base_id.as_("x"), Name("efg").fizz, Name("efg").buzz
//     #         )
//     #
//     #         test_query = SelectStatement().from_(subquery).select(subquery.x, subquery.fizz, subquery.buzz)
//     #
//     #       assert
//     #             'SELECT "sq0"."x","sq0"."fizz","sq0"."buzz" '
//     #             "FROM ("
//     #             'SELECT "base_id" "x","fizz","buzz" FROM "efg"'
//     #             ') "sq0"',
//     #             str(test_query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_join_query_with_alias(){
//     #         subquery = (
//     #             SelectStatement().from_(Name("efg"))
//     #             .select(
//     #                 Name("efg").base_id.as_("x"),
//     #                 Name("efg").fizz,
//     #                 Name("efg").buzz,
//     #             )
//     #             .as_("subq")
//     #         )
//     #
//     #         test_query = SelectStatement().from_(subquery).select(subquery.x, subquery.fizz, subquery.buzz)
//     #
//     #       assert
//     #             'SELECT "subq"."x","subq"."fizz","subq"."buzz" '
//     #             "FROM ("
//     #             'SELECT "base_id" "x","fizz","buzz" FROM "efg"'
//     #             ') "subq"',
//     #             str(test_query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_with(){
//     #         sub_query = SelectStatement().from_(Name("efg")).select("fizz")
//     #         test_query = SelectStatement().with_(sub_query, "an_alias").from_(AliasedQuery("an_alias"))
//     #
//     #       assert
//     #             'WITH an_alias AS (SELECT "fizz" FROM "efg") SELECT * FROM an_alias',
//     #             str(test_query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_join_with_with(){
//     #         sub_query = SelectStatement().from_(Name("efg")).select("fizz")
//     #         test_query = (
//     #             SelectStatement().with_(sub_query, "an_alias")
//     #             .from_(Name("abc"))
//     #             .join(AliasedQuery("an_alias"))
//     #             .on(AliasedQuery("an_alias").fizz == Name("abc").buzz)
//     #
//     #         )
//     #       assert
//     #             'WITH an_alias AS (SELECT "fizz" FROM "efg") '
//     #             'SELECT * FROM "abc" JOIN an_alias ON "an_alias"."fizz"="abc"."buzz"',
//     #             str(test_query),
//     #         )
//     #
//     #     }
//
// #[test]
// fn test_select_from_with_returning(){
//     #         sub_query = SelectStatement().into(Name("abc")).insert(1).returning('*')
//     #         test_query = SelectStatement().with_(sub_query, "an_alias").from_(AliasedQuery("an_alias"))
//     #       assert
//     #             'WITH an_alias AS (INSERT INTO "abc" VALUES (1) RETURNING *) SELECT * FROM an_alias', str(test_query)
//     #         )
//     #
//     #
