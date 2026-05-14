use crate::model::model::Model;
use crate::sequel::term::calendar::{Date, DateTime};
use orm_macros::table;

#[table("users")]
pub struct User {
    id: u32,
    #[col(char, length = 255, name = "user_name")]
    username: String,
    age: u32,
    company_id: u32,
}

#[table(name = "orders")]
pub struct Order {
    id: u32,
    user_id: u32,
    created_at: DateTime,
}

#[table(products)]
pub struct Product {
    id: u32,
    #[col(char)]
    name: String,
    expired_on: Date,
    category_id: u32,
}

#[table]
pub struct Category {
    id: u32,
    name: String,
}
#[table]
pub struct Company {
    id: u32,
    name: String,
}

#[table(name=consumer_table)]
pub struct Consumer {
    id: u32,
}

#[test]
fn test_model__base() {
    use crate::model::objects::Objects;
    let _: Objects<User> = User::objects();
}

#[test]
fn test_model__table_name() {
    assert_eq!("users", User::table_name());
    assert_eq!("orders", Order::table_name());
    assert_eq!("products", Product::table_name());
    assert_eq!("categories", Category::table_name());
    assert_eq!("companies", Company::table_name());
    assert_eq!("consumer_table", Consumer::table_name());
}

#[test]
fn test_model_char_column() {
    assert_eq!(User::user_name.length, 255);
    assert_eq!(Product::name.length, 255);
}
