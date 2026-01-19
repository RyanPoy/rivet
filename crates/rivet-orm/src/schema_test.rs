use crate::ast::expression::Expr;
use crate::ast::sql_value::{SqlValue, ToSql};
use crate::schema::Column;
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

        has_children: bool,
    }
    assert_eq!(User::COLUMNS.id.name, "id");
    assert_eq!(User::COLUMNS.username.name, "name");
    assert_eq!(User::COLUMNS.password.name, "passWord");
    assert_eq!(User::COLUMNS.age.name, "age");
    assert_eq!(
        User::COLUMNS.is_a_column_event_do_not_set_col_macro.name,
        "is_a_column_event_do_not_set_col_macro"
    );
    assert_eq!(User::COLUMNS.has_children.name, "has_children");
}

#[test]
pub fn test_column() {
    #[table(name = "users")]
    pub struct User {
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.eq("lucy");
    assert_eq!(left, "username");
    assert_eq!(op, "=");
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.eq(20);
    assert_eq!(left, "age");
    assert_eq!(op, "=");
    assert_eq!(right.to_sql(), "20");

    // let Expr::Binary { left, op, right } = User::COLUMNS.password.eq("abc".to_string());
    // assert_eq!(left, "age");
    // assert_eq!(op, "=");
    // assert_eq!(right.to_sql(), "20");
}
