use crate::model::model::Model;
use crate::model::objects::Objects;
use crate::sequel::term::calendar::{Date, DateTime};

pub struct User {
    id: u32,
    username: String,
    age: u32,
    company_id: u32,
}
pub struct Order {
    id: u32,
    user_id: u32,
    created_at: DateTime,
}
pub struct Product {
    id: u32,
    name: String,
    expired_on: Date,
    category_id: u32,
}
pub struct Category {
    id: u32,
    name: String,
}
pub struct Company {
    id: u32,
    name: String,
}

impl Model for User {
    const TABLE_NAME: &'static str = "users";
}

impl Model for Order {
    const TABLE_NAME: &'static str = "orders";
}

impl Model for Product {
    const TABLE_NAME: &'static str = "products";
}

impl Model for Category {
    const TABLE_NAME: &'static str = "categories";
}

impl Model for Company {
    const TABLE_NAME: &'static str = "companies";
}

#[test]
fn test_model() {
    let _: Objects<User> = User::objects();
    assert_eq!("users", User::table_name());
}
