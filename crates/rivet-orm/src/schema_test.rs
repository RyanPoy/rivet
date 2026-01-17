use rivet_orm_macros::table;

#[test]
fn test_table_name() {
    #[table]
    struct Teacher {}
    assert_eq!(Teacher::TABLE_NAME, "teachers");

    #[table()]
    struct Person {}
    assert_eq!(Person::TABLE_NAME, "people");

    #[table(student_lists)]
    struct STUDENT {}
    assert_eq!(STUDENT::TABLE_NAME, "student_lists");

    #[table("myCards")]
    struct Card {}
    assert_eq!(Card::TABLE_NAME, "myCards");

    #[table(name = "customers")]
    struct User {}
    assert_eq!(User::TABLE_NAME, "customers");

    #[table(name = user_profiles)]
    struct Profile {}
    assert_eq!(Profile::TABLE_NAME, "user_profiles");
}

#[test]
fn test_col() {
    #[table(name = "users")]
    struct User {
        #[col]
        id: usize,

        #[col(name = "name")]
        username: String,

        #[col(passWord)]
        password: String,

        #[col()]
        age: String,
    }
    assert_eq!(User::TABLE_NAME, "users");
    assert_eq!(User::COLUMNS, &["id", "name", "passWord", "age"]);
}
