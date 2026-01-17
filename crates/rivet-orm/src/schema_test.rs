use rivet_orm_macros::table;

#[test]
fn test_table_name() {
    #[table(name = "users")]
    struct User {}
    assert_eq!(User::TABLE_NAME, "users");

    #[table]
    struct Teacher {}
    assert_eq!(Teacher::TABLE_NAME, "teachers");

    #[table()]
    struct Person {}
    assert_eq!(Person::TABLE_NAME, "people");

    // 新增测试：检查包含下划线的情况
    #[table(name = "user_profiles")]
    struct UserProfile {}
    assert_eq!(UserProfile::TABLE_NAME, "user_profiles");

    // 测试全大写结构体名
    #[table]
    struct STUDENT {}
    assert_eq!(STUDENT::TABLE_NAME, "students");

    // 测试包含数字的结构体名
    #[table]
    struct Class2023 {}
    assert_eq!(Class2023::TABLE_NAME, "class2023s"); // 假设这种情况下的默认行为

    // 测试首字母小写的结构体名
    #[table]
    struct student {}
    assert_eq!(student::TABLE_NAME, "students"); // 验证是否总是首字母大写
}

#[test]
fn test_col() {
    #[table(name = "users")]
    struct User {
        #[col]
        id: usize,
        #[col(name = "name")]
        username: String,
        #[col]
        password: String,
    }
    assert_eq!(User::TABLE_NAME, "users");
}
