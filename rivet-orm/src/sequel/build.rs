use crate::sequel::ast::Value;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dialect {
    MySql,
    PostgreSql,
    Sqlite,
}
#[derive(Debug, Clone)]
pub struct Binder {
    dialect: Dialect,
    params: Vec<Value>,
}

impl Binder {
    pub fn new(dialect: Dialect) -> Binder {
        Binder { dialect, params: vec![] }
    }

    pub fn bind(&mut self, value: Value) -> String {
        let index = self.params.len();
        self.params.push(value);
        match self.dialect {
            Dialect::MySql | Dialect::Sqlite => "?".to_string(),
            Dialect::PostgreSql => format!("${}", index + 1),
        }
    }

    pub fn quote(&self, name: &str) -> String {
        match self.dialect {
            Dialect::MySql => format!("`{}`", name),
            Dialect::PostgreSql | Dialect::Sqlite => format!("\"{}\"", name),
        }
    }

    pub fn quote_full(&self, schema: Option<&str>, table: &str) -> String {
        match schema {
            Some(s) => format!("{}.{}", self.quote(s), self.quote(table)),
            None => self.quote(table),
        }
    }

    pub fn params(&self) -> Vec<Value> {
        self.params.clone()
    }
}
