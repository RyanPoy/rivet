use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::index::Index;
use crate::sequel::term::lock::{Lock, Wait};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::Table;
use rivet_utils::into_vec::IntoVec;

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
        self.distinct = Distinct::All;
        self
    }
    pub fn distinct_on<T, I>(mut self, cols: I) -> Self
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        let cols = cols.into_iter().map(|c| c.into()).collect();
        self.distinct = Distinct::On(cols);
        self
    }

    pub fn select(mut self, columns: impl IntoVec<SelectItem>) -> Self {
        for item in columns.into_vec() {
            self.select_clause.push(item);
        }
        self
    }

    pub fn where_(mut self, c: Expr) -> Self {
        self.where_clause.push(c);
        self
    }

    pub fn join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Clone + Into<Table>,
    {
        self.from_clause = self.from_clause.join(other, on);
        self
    }

    pub fn left_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Clone + Into<Table>,
    {
        self.from_clause = self.from_clause.left_join(other, on);
        self
    }

    pub fn right_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Clone + Into<Table>,
    {
        self.from_clause = self.from_clause.right_join(other, on);
        self
    }

    pub fn full_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Clone + Into<Table>,
    {
        self.from_clause = self.from_clause.full_join(other, on);
        self
    }
    pub fn cross_join(mut self, others: impl IntoVec<Table>) -> Self {
        let tables = others.into_vec();
        for t in tables {
            self.from_clause = self.from_clause.cross_join(&t);
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
#[path = "./select_tests/all.rs"]
mod tests;
