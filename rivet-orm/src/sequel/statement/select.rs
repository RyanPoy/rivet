use crate::prelude::*;
use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::index::Index;
use crate::sequel::term::lock::{Lock, Locking, Wait};
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
    pub locking: Option<Locking>,
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
            locking: Some(Locking::new()),
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

    pub fn where_<T>(mut self, c: T) -> Self
    where
        T: Into<Expr> + Clone,
    {
        self.where_clause.push(c.into());
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

    pub fn for_share(self) -> Self {
        self.set_lock_of_locker(Lock::Share)
    }

    pub fn for_update(self) -> Self {
        self.set_lock_of_locker(Lock::Update)
    }
    pub fn for_update_of(self, tables: impl IntoVec<Table>) -> Self {
        let tables = tables.into_vec();
        self.set_lock_of_locker(Lock::UpdateOf(tables))
    }

    fn set_lock_of_locker(mut self, lock: Lock) -> Self {
        let mut locking = self.locking.unwrap_or_else(|| Locking::new());
        locking.lock = Some(lock);
        if locking.wait.is_none() {
            locking.wait = Some(Wait::Default)
        }
        self.locking = Some(locking);
        self
    }

    pub fn wait(self) -> Self {
        self.set_wait_for_locker(Wait::Default)
    }
    pub fn no_wait(self) -> Self {
        self.set_wait_for_locker(Wait::NoWait)
    }
    pub fn skip(self) -> Self {
        self.set_wait_for_locker(Wait::SkipLocked)
    }
    fn set_wait_for_locker(mut self, wait: Wait) -> Self {
        let mut l = self.locking.unwrap_or_else(|| Locking::new());
        l.wait = Some(wait);
        self.locking = Some(l);
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

impl Comparable for SelectStatement {
    fn into_expr(&self) -> Expr {
        Expr::Subquery(Box::new(self.clone()))
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
