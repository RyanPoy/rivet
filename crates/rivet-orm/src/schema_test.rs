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
fn test_columns() {
    #[table(name = "users")]
    pub struct User {
        #[col]
        id: usize,

        #[col(name = "name")]
        username: String,

        #[col(passWord)]
        password: String,

        #[col()]
        age: u8,

        #[no_col]
        temp: String,

        is_a_column_event_do_not_set_col_macro: bool,

        has_children: bool
    }
    assert_eq!(User::COLUMNS.id.name, "id");
    assert_eq!(User::COLUMNS.username.name, "name");
    assert_eq!(User::COLUMNS.password.name, "passWord");
    assert_eq!(User::COLUMNS.age.name, "age");
    assert_eq!(User::COLUMNS.is_a_column_event_do_not_set_col_macro.name, "is_a_column_event_do_not_set_col_macro");
    assert_eq!(User::COLUMNS.has_children.name, "has_children");
}

#[test]
fn test_column() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: String,
        age: u8,
    }

    // assert_eq!(User::COLUMNS.id.eq("id"));
    // assert_eq!(User::COLUMNS.username.eq("name"));
    // assert_eq!(User::COLUMNS.password.eq("passWord"));
    // assert_eq!(User::COLUMNS.age.eq("age"));
}
