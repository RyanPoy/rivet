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
pub struct Join {
    pub name: String,
}

impl Join {
    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::Join { join: self, alias: Some(alias.into()) }
    }
}
