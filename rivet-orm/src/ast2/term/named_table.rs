use crate::ast2::term::table_ref::TableRef;

#[derive(Debug, Clone)]
pub struct NamedTable(String);

impl<T> From<T> for NamedTable
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        NamedTable::new(value)
    }
}

impl NamedTable {
    pub fn new(name: impl Into<String>) -> Self {
        NamedTable(name.into())
    }

    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::NamedTable { table: self, alias: Some(alias.into()) }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.as_str()
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }
}
