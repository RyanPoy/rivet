
#[test]
fn test_table_name() {
    use rivet_orm_macros::table;

    #[table(name = "users")]
    struct User {}
}