use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::join::{Join, JoinType};
use rivet_utils::impl_into_vec_for;
use rivet_utils::into_vec::IntoVec;
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
impl IntoVec<Table> for &Table {
    fn into_vec(self) -> Vec<Table> {
        vec![self.clone()]
    }
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

    pub fn fork(&self) -> Self {
        let inner = (*self.inner).clone();
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

    fn _join<T>(self, other: &T, join_type: JoinType, on: Option<Expr>) -> Self
    where
        T: Into<Table> + Clone,
    {
        let inner = TableInner::Join(Join {
            left: Box::new(self),
            right: Box::new(other.clone().into()),
            join_type,
            on,
        });
        Self {
            inner: Arc::new(inner),
            alias: None,
        }
    }

    pub fn join<T>(self, other: &T, on: Expr) -> Self
    where
        T: Into<Table> + Clone,
    {
        self._join(other, JoinType::Inner, Some(on))
    }
    pub fn left_join<T>(self, other: &T, on: Expr) -> Self
    where
        T: Into<Table> + Clone,
    {
        self._join(other, JoinType::Left, Some(on))
    }
    pub fn right_join<T>(self, other: &T, on: Expr) -> Self
    where
        T: Into<Table> + Clone,
    {
        self._join(other, JoinType::Right, Some(on))
    }
    pub fn full_join<T>(self, other: &T, on: Expr) -> Self
    where
        T: Into<Table> + Clone,
    {
        self._join(other, JoinType::Full, Some(on))
    }
    pub fn cross_join<T>(self, other: &T) -> Self
    where
        T: Into<Table> + Clone,
    {
        self._join(other, JoinType::Cross, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_fork() {
        let u = Table::new("foo");
        let u_clone = u.clone();
        let u_fork = u.fork();

        assert!(Arc::ptr_eq(&u.inner, &u_clone.inner));
        assert!(!Arc::ptr_eq(&u.inner, &u_fork.inner));
    }
}
