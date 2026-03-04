use crate::ast2::term::alias::Alias;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::join::{Join, JoinType};
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::subquery::Subquery;

#[derive(Debug, Clone)]
pub enum TableRef {
    Named { table: NamedTable, alias: Option<Alias> },
    Subquery { subquery: Subquery, alias: Alias },
    Join { join: Join, alias: Option<Alias> },
}

impl<T> From<T> for TableRef
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::Named {
            table: NamedTable::new(value),
            alias: None,
        }
    }
}

impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::Named {
            table: value,
            alias: None,
        }
    }
}

impl TableRef {
    pub fn column(&self, name: impl Into<String>) -> ColumnRef {
        ColumnRef {
            qualifier: Some(self.visible_name().to_string()),
            name: name.into(),
        }
    }
    pub fn alias(self, name: impl Into<Alias>) -> Self {
        match self {
            Self::Named { table, .. } => Self::Named {
                table,
                alias: Some(name.into()),
            },
            Self::Subquery { subquery, .. } => Self::Subquery {
                subquery,
                alias: name.into(),
            },
            Self::Join { join, .. } => Self::Join {
                join,
                alias: Some(name.into()),
            },
        }
    }

    pub fn visible_name(&self) -> &str {
        match self {
            Self::Named { table, alias } => alias.as_ref().map_or(table.name(), |a| a.name()),
            Self::Subquery { subquery, alias } => alias.name(),
            Self::Join { join, alias } => alias.as_ref().map_or(&join.visible_name(), |a| a.name()),
        }
    }

    pub fn join(self, other: impl Into<TableRef>, join_type: JoinType, on: Option<Expr>) -> Self {
        let join = Join {
            left: Box::new(self),
            right: Box::new(other.into()),
            join_type,
            on,
        };
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
