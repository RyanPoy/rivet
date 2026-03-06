use crate::ast2::term::alias::Alias;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::table_ref::{TableInner, TableRef};
use std::sync::Arc;

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
    pub fn alias(self, alias: impl Into<Alias>) -> TableRef {
        let inner = TableInner::Join(self);
        TableRef {
            inner: Arc::new(inner),
            alias: Some(alias.into()),
        }
    }

    pub fn visible_name_or(&self, default: usize) -> String {
        self.right.visible_name_or(default)
    }
}
