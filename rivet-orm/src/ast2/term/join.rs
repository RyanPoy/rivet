use crate::ast2::term::expr::Expr;
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
    pub left: Box<TableRef>,
    pub join_type: JoinType,
    pub right: Box<TableRef>,
    pub on: Option<Expr>,
}

impl Join {
    pub fn alias(self, alias: impl Into<String>) -> TableRef {
        TableRef::Join { join: self, alias: Some(alias.into()) }
    }

    pub fn visible_name(&self) -> &str {
        self.right.visible_name()
    }
}
