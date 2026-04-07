use std::sync::LazyLock;
use crate::model::model::Model;
use crate::sequel::term::calendar::{Date, DateTime};
use crate::sequel::term::literal::Literal;
use crate::sequel::term::table::Table;

pub fn compare_params(actual: Vec<Literal>, expected: Vec<Literal>) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    for (a, e) in actual.iter().zip(expected.iter()) {
        match (a, e) {
            (Literal::Int(a), Literal::Int(e)) if a == e => continue,
            (Literal::Float(a), Literal::Float(e)) if a == e => continue,
            (Literal::String(a), Literal::String(e)) if a == e => continue,
            (Literal::Bool(a), Literal::Bool(e)) if a == e => continue,
            (Literal::Null, Literal::Null) => continue,
            _ => return false,
        }
    }
    true
}

macro_rules! assert_mysql {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::mysql().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert!(super::helper::compare_params(params, expected), "params mismatch");
    };
}

macro_rules! assert_pg {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::postgre().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert!(super::helper::compare_params(params, expected), "params mismatch");
    };
}

macro_rules! assert_sqlite {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::sqlite().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert!(super::helper::compare_params(params, expected), "params mismatch");
    };
}

// ============================================================================
// 基础表定义
// ============================================================================

pub static USERS: LazyLock<Table> = LazyLock::new(|| Table::new("users"));
pub static ORDERS: LazyLock<Table> = LazyLock::new(|| Table::new("orders"));
pub static PRODUCTS: LazyLock<Table> = LazyLock::new(|| Table::new("products"));
pub static CATEGORIES: LazyLock<Table> = LazyLock::new(|| Table::new("categories"));
pub static COMPANIES: LazyLock<Table> = LazyLock::new(|| Table::new("companies"));

pub static MYTABLE: LazyLock<Table> = LazyLock::new(|| Table::new("mytable"));
pub static MYOTHERTABLE: LazyLock<Table> = LazyLock::new(|| Table::new("myothertable"));
pub static PARENT: LazyLock<Table> = LazyLock::new(|| Table::new("parent"));
pub static CHILD: LazyLock<Table> = LazyLock::new(|| Table::new("child"));
pub static GRANDCHILD: LazyLock<Table> = LazyLock::new(|| Table::new("grandchild"));
pub static GRANDCHILD_W_PARENT: LazyLock<Table> = LazyLock::new(|| Table::new("grandchildwparent"));
pub static TBL: LazyLock<Table> = LazyLock::new(|| Table::new("tbl"));

pub struct User { id: u32, username: String, age: u32, company_id: u32 }
pub struct Order { id: u32, user_id: u32, created_at: DateTime }
pub struct Product { id: u32, name: String, expired_on: Date, category_id: u32 }
pub struct Category { id: u32, name: String, }
pub struct Company { id: u32, name: String }

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
