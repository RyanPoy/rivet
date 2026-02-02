use crate::sequel::ast::{Scalar, Value};
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dialect {
    MySql,
    PostgreSql,
    Sqlite,
}
#[derive(Debug, Clone)]
pub struct Binder {
    dialect: Dialect,
    params: Vec<Scalar>,
}

impl Binder {
    pub fn new(dialect: Dialect) -> Binder {
        Binder { dialect, params: vec![] }
    }

    pub fn bind(&mut self, value: Scalar) -> String {
        let index = self.params.len();
        self.params.push(value);
        match self.dialect {
            Dialect::MySql | Dialect::Sqlite => "?".to_string(),
            Dialect::PostgreSql => format!("${}", index + 1),
        }
    }

    pub fn format_literal(&self, value: Scalar) -> String {
        match value {
            Scalar::Null => String::from("NULL"),
            Scalar::I8(v) => v.to_string(),
            Scalar::I16(v) => v.to_string(),
            Scalar::I32(v) => v.to_string(),
            Scalar::I64(v) => v.to_string(),
            Scalar::I128(v) => v.to_string(),
            Scalar::U8(v) => v.to_string(),
            Scalar::U16(v) => v.to_string(),
            Scalar::U32(v) => v.to_string(),
            Scalar::U64(v) => v.to_string(),
            Scalar::U128(v) => v.to_string(),
            Scalar::Bool(v) => v.to_string(),
            Scalar::String(v) => self.quote(&v),
        }
    }

    pub fn quote(&self, name: &str) -> String {
        match self.dialect {
            Dialect::MySql => format!("`{}`", name),
            Dialect::PostgreSql | Dialect::Sqlite => format!("\"{}\"", name),
        }
    }

    pub fn quote_full(&self, prefix: Option<&str>, table: &str) -> String {
        match prefix {
            Some(s) => format!("{}.{}", self.quote(s), self.quote(table)),
            None => self.quote(table),
        }
    }

    pub fn with_alias(&self, sql: String, alias: Option<&str>) -> String {
        match alias {
            Some(alias) => format!("{} AS {}", sql, self.quote(alias)),
            None => sql,
        }
    }
    pub fn params(&self) -> Vec<Scalar> {
        self.params.clone()
    }
}
