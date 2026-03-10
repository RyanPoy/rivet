use crate::sequel::term::column_ref::ColumnRef;
use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::index::Index;
use crate::sequel::term::lock::{Lock, Wait};
use crate::sequel::term::select_item::{IntoSelectItems, SelectItem};
use crate::sequel::term::table::{IntoTables, Table};

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub distinct: Distinct,
    pub select_clause: Vec<SelectItem>,
    pub from_clause: Table,
    pub where_clause: Vec<Expr>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub locking: Option<(Lock, Wait)>,
    pub indexes: Vec<Index>,
}

impl SelectStatement {
    pub fn from<T>(t: &T) -> Self
    where
        T: Clone + Into<Table>,
    {
        Self {
            distinct: Distinct::None,
            select_clause: Vec::new(),
            from_clause: t.clone().into(),
            where_clause: Vec::new(),
            limit: None,
            offset: None,
            locking: None,
            indexes: Vec::new(),
        }
    }
    pub fn distinct(mut self) -> Self {
        self.distinct = Distinct::Simple;
        self
    }
    pub fn distinct_on(mut self, cols: Vec<ColumnRef>) -> Self {
        self.distinct = Distinct::On(cols);
        self
    }

    pub fn select<C>(mut self, c: C) -> Self
    where
        C: IntoSelectItems,
    {
        self.select_clause.extend(c.into_select_items());
        self
    }

    pub fn where_(mut self, c: Expr) -> Self {
        self.where_clause.push(c);
        self
    }

    pub fn join(mut self, other: impl Into<Table>, on: Expr) -> Self {
        self.from_clause = self.from_clause.inner_join(other, on);
        self
    }
    pub fn left_join(mut self, other: impl Into<Table>, on: Expr) -> Self {
        self.from_clause = self.from_clause.left_join(other, on);
        self
    }
    pub fn right_join(mut self, other: impl Into<Table>, on: Expr) -> Self {
        self.from_clause = self.from_clause.right_join(other, on);
        self
    }
    pub fn full_join(mut self, other: impl Into<Table>, on: Expr) -> Self {
        self.from_clause = self.from_clause.full_join(other, on);
        self
    }
    pub fn cross_join(mut self, others: impl IntoTables) -> Self {
        let tables = others.into_table_refs();
        for t in tables {
            self.from_clause = self.from_clause.cross_join(t);
        }
        self
    }

    pub fn for_update(mut self, lock: Lock, wait: Wait) -> Self {
        self.locking = Some((lock, wait));
        self
    }

    pub fn force_index<T>(mut self, index: T) -> Self
    where
        T: Into<Index>,
    {
        self.indexes.push(index.into());
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        if n > 0 {
            self.limit = Some(n);
        }
        self
    }

    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }

    pub fn alias(self, name: &str) -> Table {
        Table::from(self).alias(name)
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;

#[cfg(test)]
#[path = "./select_test2.rs"]
mod select_tests;
