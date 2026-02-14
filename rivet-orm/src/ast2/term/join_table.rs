use crate::ast2::term::table_ref::TableRef;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

#[derive(Debug, Clone)]
pub struct JoinedTable {
    name: Option<String>,
}

impl JoinedTable {
    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::JoinedTable { table: self, alias: Some(alias.into()) }
    }
}
