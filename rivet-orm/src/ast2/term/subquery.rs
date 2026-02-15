use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::table_ref::TableRef;

#[derive(Debug, Clone)]
pub struct Subquery {
    pub stmt: Box<SelectStatement>,
}

impl Subquery {
    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::Subquery { table: self, alias: alias.into() }
    }
}
