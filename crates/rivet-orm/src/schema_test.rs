
#[test]
fn test_table_name() {
    use rivet_orm_macros::table;

    #[table(name = "users")]
    struct User {}
    assert_eq!(User::TABLE_NAME, "users");

    #[table]
    struct Teacher {}
    assert_eq!(Teacher::TABLE_NAME, "teachers");
}