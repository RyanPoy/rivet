use rivet_utils::into_vec::IntoVec;
use crate::model::model::Model;
use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::lock::{Lock, Wait};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::Table;


pub struct Objects<M> {
    select: SelectStatement,
    _marker: std::marker::PhantomData<M>,
}

impl<M> Objects<M>
where
    M: Model,
{
    pub fn new(table: &Table) -> Self {
        Self {select: SelectStatement::from(table), _marker: std::marker::PhantomData }
    }

    pub fn distinct(mut self) -> Self {
        self
    }

    pub fn distinct_on<T, I>(mut self, cols: I) -> Self {
        self
    }
    pub fn select(mut self, c: impl Into<SelectItem>) -> Self {
        self
    }
    pub fn where_(mut self, c: Expr) -> Self {
        self
    }
    pub fn join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Into<Table>,
    {
        self
    }
    pub fn left_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Into<Table>,
    {
        self
    }
    pub fn right_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Into<Table>,
    {
        self
    }
    pub fn full_join<T>(mut self, other: &T, on: Expr) -> Self
    where
        T: Into<Table>,
    {
        self
    }
    pub fn cross_join(mut self, others: impl IntoVec<Table>) -> Self
    {
        self
    }
    pub fn for_update(mut self, lock: Lock, wait: Wait) -> Self
    {
        self
    }
    pub fn force_index<T>(mut self, index: T) -> Self
    where
        T: Into<Table>,
    {
        self
    }
    pub fn limit(mut self, n: usize) -> Self {
        self
    }
    pub fn offset(mut self, n: usize) -> Self {
        self
    }
    // pub fn alias(self, name: &str) -> Table {
    //     self
    // }
}
