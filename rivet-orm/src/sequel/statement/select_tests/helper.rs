use crate::sequel::term::table::Table;
use std::sync::LazyLock;

macro_rules! assert_mysql {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::mysql().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
    };
}

macro_rules! assert_pg {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::postgre().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
    };
}

macro_rules! assert_sqlite {
    ($stmt:expr, $expected_sql:expr, [$($params:expr),*]) => {
        let (sql, params) = crate::sequel::visitor::visitor::sqlite().visit_select_statement($stmt).finish();
        assert_eq!(sql, $expected_sql.to_string());
        let expected: Vec<crate::sequel::term::literal::Literal> = vec![$($params.into()),*];
        assert_eq!(params, expected);
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
