use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::table_ref::TableRef;

#[derive(Debug, Clone)]
pub struct DerivedTable {
    pub stmt: Box<SelectStatement>,
}

impl DerivedTable {
    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::DerivedTable { table: self, alias: alias.into() }
    }
}
