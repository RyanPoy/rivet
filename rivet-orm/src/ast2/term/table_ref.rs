use crate::ast2::term::alias::Alias;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::join::{Join, JoinType};
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::subquery::Subquery;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TableRef {
    pub inner: Arc<TableInner>,
    pub alias: Option<Alias>,
}

#[derive(Debug, Clone)]
pub enum TableInner {
    Named(NamedTable),
    Subquery(Subquery),
    Join(Join),
}

impl<T> From<T> for TableRef
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let inner = TableInner::Named(NamedTable::new(value));
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }
}

impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        let inner = TableInner::Named(value);
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }
}

impl TableRef {
    pub fn column(&self, name: impl Into<String>) -> ColumnRef {
        ColumnRef {
            name: name.into(),
            table_inner: Some(self.inner.clone()),
        }
    }
    pub fn alias(mut self, name: impl Into<String>) -> Self {
        self.alias = Some(Alias::new(name.into()));
        self
    }

    pub fn visible_name_or(&self, default: usize) -> String {
        if let Some(alias) = &self.alias {
            alias.name().to_string()
        } else {
            format!("t{}", default)
        }
    }

    pub fn join(self, other: impl Into<TableRef>, join_type: JoinType, on: Option<Expr>) -> Self {
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

    pub fn inner_join(self, other: impl Into<TableRef>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Inner, on)
    }
    pub fn left_join(self, other: impl Into<TableRef>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Left, on)
    }
    pub fn right_join(self, other: impl Into<TableRef>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Right, on)
    }
    pub fn full_join(self, other: impl Into<TableRef>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Full, on)
    }
    pub fn cross_join(self, other: impl Into<TableRef>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Cross, on)
    }
}
