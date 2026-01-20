use super::*;
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
pub fn test_column_eq() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }

    let Expr::Binary { left, op, right } = User::COLUMNS.username.eq("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Eq);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.eq(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Eq);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.eq("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Eq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.eq("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Eq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.eq(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Eq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.eq(None) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Is);
    assert_eq!(right.to_sql(), "NULL");
}
#[test]
pub fn test_column_neq() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.neq("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Neq);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.neq(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Neq);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.neq("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Neq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.neq("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Neq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.neq(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Neq);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.neq(None) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::IsNot);
    assert_eq!(right.to_sql(), "NULL");
}

#[test]
pub fn test_column_gt() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.gt("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Gt);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.gt(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Gt);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gt("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gt);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gt("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gt);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gt(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gt);
    assert_eq!(right.to_sql(), "123qwe");

    assert!(User::COLUMNS.password.gt(None).is_empty());
}

#[test]
pub fn test_column_gte() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.gte("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Gte);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.gte(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Gte);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gte("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gte);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gte("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gte);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.gte(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Gte);
    assert_eq!(right.to_sql(), "123qwe");

    assert!(User::COLUMNS.password.gte(None).is_empty());
}

#[test]
pub fn test_column_lt() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.lt("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Lt);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.lt(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Lt);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lt("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lt);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lt("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lt);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lt(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lt);
    assert_eq!(right.to_sql(), "123qwe");

    assert!(User::COLUMNS.password.lt(None).is_empty());
}

#[test]
pub fn test_column_lte() {
    #[table(name = "users")]
    pub struct User {
        id: usize,
        username: String,
        password: Option<String>,
        age: u8,
    }
    let Expr::Binary { left, op, right } = User::COLUMNS.username.lte("lucy") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "username");
    assert_eq!(op, Op::Lte);
    assert_eq!(right.to_sql(), "lucy");

    let Expr::Binary { left, op, right } = User::COLUMNS.age.lte(20) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "age");
    assert_eq!(op, Op::Lte);
    assert_eq!(right.to_sql(), "20");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lte("123qwe") else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lte);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lte("123qwe".to_string()) else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lte);
    assert_eq!(right.to_sql(), "123qwe");

    let Expr::Binary { left, op, right } = User::COLUMNS.password.lte(Some("123qwe".to_string()))
    else {
        panic!("Should not process: expected Expr::Binary, but got Expr::Empty.");
    };
    assert_eq!(left, "password");
    assert_eq!(op, Op::Lte);
    assert_eq!(right.to_sql(), "123qwe");

    assert!(User::COLUMNS.password.lte(None).is_empty());
}
