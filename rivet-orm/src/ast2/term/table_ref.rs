use crate::ast2::term::derived_table::DerivedTable;
use crate::ast2::term::join_table::JoinedTable;
use crate::ast2::term::named_table::NamedTable;

#[derive(Debug, Clone)]
pub enum TableRef {
    NamedTable(NamedTable),
    DerivedTable(DerivedTable),
    JoinedTable(JoinedTable),
}

impl From<&str> for TableRef {
    fn from(value: &str) -> Self {
        Self::NamedTable(NamedTable { name: value.to_string(), alias: None })
    }
}
impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::NamedTable(value)
    }
}

impl TableRef {
    pub fn alias(self, name: impl Into<String>) -> Self {
        match self {
            Self::NamedTable(t) => Self::NamedTable(t.alias(name)),
            Self::DerivedTable(t) => Self::DerivedTable(t.alias(name)),
            Self::JoinedTable(t) => Self::JoinedTable(t.alias(name)),
        }
    }
}
