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
    #[inline]
    pub fn push(&mut self, s: &str) -> &mut Self {
        self.buff.push_str(s);
        self
    }

    #[inline]
    pub fn push_alias(&mut self, alias: Option<&str>) -> &mut Self {
        if let Some(a) = alias {
            self.push(" AS ");
            self.push_quote(a);
        }
        self
    }

    pub fn push_quote(&mut self, s: &str) -> &mut Self {
        match self.dialect {
            Dialect::MySQL => self.push("`").push(s).push("`"),
            Dialect::PostgreSQL | Dialect::SQLite => self.push("\"").push(s).push("\""),
        }
    }
    #[inline]
    pub fn push_with_alias(&mut self, s: &str, alias: Option<&str>) -> &mut Self {
        self.push(s).push_alias(alias)
    }

    #[inline]
    pub fn push_quote_with_alias(&mut self, s: &str, alias: Option<&str>) -> &mut Self {
        self.push_quote(s).push_alias(alias)
    }

    pub fn clear(&mut self) -> &mut Self {
        self.buff.clear();
        self
    }
}
