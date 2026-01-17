use rivet_orm_macros::table;

#[test]
fn test_table_name() {
    #[table]
    struct Teacher {}
    assert_eq!(Teacher::TABLE_NAME, "teachers");

    #[table()]
    struct Person {}
    assert_eq!(Person::TABLE_NAME, "people");

    #[table(students)]
    struct STUDENT {}
    assert_eq!(STUDENT::TABLE_NAME, "students");

    #[table("cards")]
    struct Card {}
    assert_eq!(Card::TABLE_NAME, "cards");

    #[table(name = "users")]
    struct User {}
    assert_eq!(User::TABLE_NAME, "users");

    #[table(name = profiles)]
    struct Profile {}
    assert_eq!(Profile::TABLE_NAME, "profiles");
}

//
// #[test]
// fn test_col() {
//     #[table(name = "users")]
//     struct User {
//         #[cold]
//         id: usize,
//         #[col(name = "name")]
//         username: String,
//         #[col]
//         password: String,
//     }
//     assert_eq!(User::TABLE_NAME, "users");
// }
