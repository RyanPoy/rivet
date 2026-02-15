use crate::ast2::term::join::Join;
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::subquery::Subquery;

#[derive(Debug, Clone)]
pub enum TableRef {
    NamedTable { table: NamedTable, alias: Option<String> },
    Subquery { table: Subquery, alias: String },
    Join { table: Join, alias: Option<String> },
}

impl<T> From<T> for TableRef
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::NamedTable { table: NamedTable::new(value), alias: None }
    }
}

impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::NamedTable { table: value, alias: None }
    }
}

impl TableRef {
    pub fn alias(self, value: impl Into<String>) -> Self {
        match self {
            Self::NamedTable { table, .. } => Self::NamedTable { table, alias: Some(value.into()) },
            Self::Subquery { table, .. } => Self::Subquery { table, alias: value.into() },
            Self::Join { table, .. } => Self::Join { table, alias: Some(value.into()) },
        }
    }

    pub fn visible_name(&self) -> &str {
        match self {
            Self::NamedTable { table, alias } => {
                if let Some(a) = alias {
                    a.as_str()
                } else {
                    table.name()
                }
            }

            Self::Subquery { table, alias } => alias,

            Self::Join { table, alias } => {
                if let Some(a) = alias {
                    a.as_str()
                } else {
                    table.name.as_str()
                }
            }
        }
    }
}
