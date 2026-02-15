use crate::ast2::term::derived_table::DerivedTable;
use crate::ast2::term::join_table::JoinedTable;
use crate::ast2::term::named_table::NamedTable;

#[derive(Debug, Clone)]
pub enum TableRef {
    NamedTable { table: NamedTable, alias: Option<String> },
    DerivedTable { table: DerivedTable, alias: Option<String> },
    JoinedTable { table: JoinedTable, alias: Option<String> },
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
            Self::DerivedTable { table, .. } => Self::DerivedTable { table, alias: Some(value.into()) },
            Self::JoinedTable { table, .. } => Self::JoinedTable { table, alias: Some(value.into()) },
        }
    }

    // pub fn visible_name(&self) -> &str {
    //     match self {
    //         Self::NamedTable { table, alias } => {
    //             if let Some(a) = alias {
    //                 a.as_str()
    //             } else {
    //                 table.name.as_str()
    //             }
    //         }
    //
    //         Self::DerivedTable { table, alias } => alias.as_deref().expect("DerivedTable miss alias"),
    //
    //         Self::JoinedTable { table, alias } => {
    //             if let Some(a) = alias {
    //                 a.as_str()
    //             } else {
    //                 table.name.as_str()
    //             }
    //         }
    //     }
    // }
}
