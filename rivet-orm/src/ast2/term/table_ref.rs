use crate::ast2::term::expr::Expr;
use crate::ast2::term::join::{Join, JoinType};
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::subquery::Subquery;

#[derive(Debug, Clone)]
pub enum TableRef {
    Named { table: NamedTable, alias: Option<String> },
    Subquery { subquery: Subquery, alias: String },
    Join { join: Join, alias: Option<String> },
}

impl<T> From<T> for TableRef
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::Named { table: NamedTable::new(value), alias: None }
    }
}

impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::Named { table: value, alias: None }
    }
}

impl TableRef {
    pub fn alias(self, value: impl Into<String>) -> Self {
        match self {
            Self::Named { table, .. } => Self::Named { table, alias: Some(value.into()) },
            Self::Subquery { subquery, .. } => Self::Subquery { subquery, alias: value.into() },
            Self::Join { join, .. } => Self::Join { join, alias: Some(value.into()) },
        }
    }

    pub fn visible_name(&self) -> &str {
        match self {
            Self::Named { table, alias } => alias.as_deref().unwrap_or_else(|| table.name()),
            Self::Subquery { subquery, alias } => alias,
            Self::Join { join, alias } => alias.as_deref().unwrap_or_else(|| &join.visible_name()),
        }
    }

    pub fn join(self, other: impl Into<TableRef>, join_type: JoinType, on: Option<Expr>) -> Self {
        let join = Join { left: Box::new(self), right: Box::new(other.into()), join_type, on };
        Self::Join { join, alias: None }
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
