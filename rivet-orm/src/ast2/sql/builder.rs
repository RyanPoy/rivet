use crate::ast2::sql::dialect::Dialect;

pub struct Builder {
    dialect: Dialect,
    pub buff: String,
}

impl Builder {
    pub fn new(dialect: Dialect) -> Self {
        Self::with_capacity(dialect, 512)
    }
    pub fn with_capacity(dialect: Dialect, size: usize) -> Self {
        Self { dialect, buff: String::with_capacity(size) }
    }
    pub fn push(&mut self, s: &str) {
        self.buff.push_str(s);
    }

    pub fn push_quote(&mut self, s: &str) {
        match self.dialect {
            Dialect::MySQL => {
                self.push("`");
                self.push(s);
                self.push("`");
            }
            Dialect::PostgreSQL | Dialect::SQLite => {
                self.push("\"");
                self.push(s);
                self.push("\"");
            }
        }
    }
    pub fn push_with_alias(&mut self, s: &str, alias: Option<&str>) {
        self.push_quote(s);
        if let Some(a) = alias {
            self.push(" AS ");
            self.push_quote(a);
        }
    }
    pub fn clear(&mut self) {
        self.buff.clear();
    }
}
