use rivet_orm_macros::table;

#[test]
fn test_table_name() {
    #[table]
    pub struct Teacher {}
    assert_eq!(Teacher::TABLE_NAME, "teachers");

    #[table()]
    pub struct Person {}
    assert_eq!(Person::TABLE_NAME, "people");

    #[table(student_lists)]
    pub struct STUDENT {}
    assert_eq!(STUDENT::TABLE_NAME, "student_lists");

    #[table("myCards")]
    pub struct Card {}
    assert_eq!(Card::TABLE_NAME, "myCards");

    #[table(name = "customers")]
    pub struct User {}
    assert_eq!(User::TABLE_NAME, "customers");

    #[table(name = user_profiles)]
    pub struct Profile {}
    assert_eq!(Profile::TABLE_NAME, "user_profiles");
}

#[test]
fn test_col() {
    #[table(name = "users")]
    pub struct User {
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
    assert_eq!(User::COLUMNS.id.name, "id");
    assert_eq!(User::COLUMNS.username.name, "name");
    assert_eq!(User::COLUMNS.password.name, "passWord");
    assert_eq!(User::COLUMNS.age.name, "age");
}
