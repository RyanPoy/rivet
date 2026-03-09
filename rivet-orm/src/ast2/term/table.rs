use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::join::{Join, JoinType};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Table {
    pub inner: Arc<TableInner>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TableInner {
    Named(String),
    Subquery(Box<SelectStatement>),
    Join(Join),
}

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

    pub fn column(&self, name: impl Into<String>) -> ColumnRef {
        ColumnRef {
            name: name.into(),
            table_inner: Some(self.inner.clone()),
        }
    }

    pub fn columns<T, I>(&self, names: I) -> Vec<ColumnRef>
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        names.into_iter().map(|e| self.column(e)).collect()
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

    pub fn join(self, other: impl Into<Table>, join_type: JoinType, on: Option<Expr>) -> Self {
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

    pub fn inner_join(self, other: impl Into<Table>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Inner, on)
    }
    pub fn left_join(self, other: impl Into<Table>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Left, on)
    }
    pub fn right_join(self, other: impl Into<Table>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Right, on)
    }
    pub fn full_join(self, other: impl Into<Table>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Full, on)
    }
    pub fn cross_join(self, other: impl Into<Table>, on: Option<Expr>) -> Self {
        self.join(other, JoinType::Cross, on)
    }
}

pub trait IntoTableRefs {
    fn into_table_refs(self) -> Vec<Table>;
}

impl IntoTableRefs for Table {
    fn into_table_refs(self) -> Vec<Table> {
        vec![self]
    }
}
impl IntoTableRefs for &Table {
    fn into_table_refs(self) -> Vec<Table> {
        vec![self.clone()]
    }
}

impl IntoTableRefs for &str {
    fn into_table_refs(self) -> Vec<Table> {
        vec![self.into()]
    }
}

impl<T> IntoTableRefs for Vec<T>
where
    T: Into<Table>,
{
    fn into_table_refs(self) -> Vec<Table> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T, const N: usize> IntoTableRefs for [T; N]
where
    T: Into<Table>,
{
    fn into_table_refs(self) -> Vec<Table> {
        self.into_iter().map(Into::into).collect()
    }
}
