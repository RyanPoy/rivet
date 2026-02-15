use crate::ast2::term::table_ref::TableRef;

#[derive(Debug, Clone)]
pub struct NamedTable {
    pub name: String,
}
impl From<&str> for NamedTable {
    fn from(value: &str) -> Self {
        NamedTable::new(value)
    }
}

impl NamedTable {
    pub fn new(name: impl Into<String>) -> Self {
        NamedTable { name: name.into() }
    }

    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::NamedTable { table: self, alias: Some(alias.into()) }
    }
}
