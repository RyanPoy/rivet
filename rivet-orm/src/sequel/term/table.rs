use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::join::{Join, JoinType};
use rivet_utils::impl_into_vec_for;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum TableInner {
    Named(String),
    Subquery(Box<SelectStatement>),
    Join(Join),
}

#[derive(Debug, Clone)]
pub struct Table {
    pub inner: Arc<TableInner>,
    pub alias: Option<String>,
}
impl_into_vec_for!(Table => [Table]);

impl From<&str> for Table {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
impl From<SelectStatement> for Table {
    fn from(value: SelectStatement) -> Self {
        let inner = TableInner::Subquery(Box::new(value));
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }
}

impl Table {
    pub fn new(value: impl Into<String>) -> Self {
        let inner = TableInner::Named(value.into());
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }

    pub fn column(&self, name: impl Into<String>) -> Column {
        Column {
            name: name.into(),
            table_inner: self.inner.clone(),
        }
    }

    pub fn alias(mut self, name: impl Into<String>) -> Self {
        self.alias = Some(name.into());
        self
    }

    pub fn visible_name_or(&self, default: usize) -> String {
        if let Some(alias) = &self.alias {
            alias.clone()
        } else {
            format!("t{}", default)
        }
    }

    fn join(self, other: impl Into<Table>, join_type: JoinType, on: Option<Expr>) -> Self {
        let inner = TableInner::Join(Join {
            left: Box::new(self),
            right: Box::new(other.into()),
            join_type,
            on,
        });
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }

    pub fn inner_join(self, other: impl Into<Table>, on: Expr) -> Self {
        self.join(other, JoinType::Inner, Some(on))
    }
    pub fn left_join(self, other: impl Into<Table>, on: Expr) -> Self {
        self.join(other, JoinType::Left, Some(on))
    }
    pub fn right_join(self, other: impl Into<Table>, on: Expr) -> Self {
        self.join(other, JoinType::Right, Some(on))
    }
    pub fn full_join(self, other: impl Into<Table>, on: Expr) -> Self {
        self.join(other, JoinType::Full, Some(on))
    }
    pub fn cross_join(self, other: impl Into<Table>) -> Self {
        self.join(other, JoinType::Cross, None)
    }
}
