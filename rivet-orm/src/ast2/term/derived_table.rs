use crate::ast2::statement::select::SelectStatement;

#[derive(Debug, Clone)]
pub struct DerivedTable {
    pub stmt: SelectStatement,
    pub alias: Option<String>,
}

impl DerivedTable {
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
}
