use crate::ast2::term::join::Join;
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::subquery::Subquery;

#[derive(Debug, Clone)]
pub enum TableRef {
    Named { table: NamedTable, alias: Option<String> },
    Subquery { subquery: Subquery, alias: String },
    Join { join: Join, alias: Option<String> },
}

impl<T> From<T> for TableRef
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::Named { table: NamedTable::new(value), alias: None }
    }
}

impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::Named { table: value, alias: None }
    }
}

impl TableRef {
    pub fn alias(self, value: impl Into<String>) -> Self {
        match self {
            Self::Named { table, .. } => Self::Named { table, alias: Some(value.into()) },
            Self::Subquery { subquery, .. } => Self::Subquery { subquery, alias: value.into() },
            Self::Join { join, .. } => Self::Join { join, alias: Some(value.into()) },
        }
    }

    pub fn visible_name(&self) -> &str {
        match self {
            Self::Named { table, alias } => alias.as_deref().unwrap_or_else(|| table.name()),
            Self::Subquery { subquery, alias } => alias,
            Self::Join { join, alias } => alias.as_deref().unwrap_or_else(|| &join.name),
        }
    }
}
