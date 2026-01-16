use rivet_orm_macros::table;

#[test]
fn test_table_name() {
    #[table(name = "users")]
    struct User {}

    assert_eq!(User::table_name, "users")
}
