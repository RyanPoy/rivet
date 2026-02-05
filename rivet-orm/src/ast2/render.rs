pub enum Dialect {
    MySQL,
    PostgreSQL,
    SQLite,
}
pub struct SqlRender {
    dialect: Dialect,
}

impl SqlRender {
    pub fn mysql() -> Self {
        Self { dialect: Dialect::MySQL }
    }

    pub fn postgre() -> Self {
        Self { dialect: Dialect::PostgreSQL }
    }

    pub fn sqlite() -> Self {
        Self { dialect: Dialect::SQLite }
    }

    pub fn quote(&self, s: &str) -> String {
        match self.dialect {
            Dialect::MySQL => format!("`{}`", s),
            Dialect::PostgreSQL => format!("\"{}\"", s),
            Dialect::SQLite => format!("\"{}\"", s),
        }
    }
}
