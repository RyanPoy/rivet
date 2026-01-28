use crate::ast::SelectStatement;
use crate::ast::Expr;


#[derive(Clone)]
pub enum Source {
    Table { name: &'static str, alias: Option<&'static str> },
    SubQuery { query: Box<SelectStatement>, alias: &'static str },
    Join { left: Box<Source>, right: Box<Source>, tp: JoinType, on: Expr },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl Source {
    pub fn join(self, other: Source, tp: JoinType, on: Expr) -> Self {
        Source::Join { left: Box::new(self), right: Box::new(other), tp, on }
    }
    pub fn inner_join(self, other: Source, on: Expr) -> Self {
        self.join(other, JoinType::Inner, on)
    }
    pub fn left_join(self, other: Source, on: Expr) -> Self {
        self.join(other, JoinType::Left, on)
    }
    pub fn right_join(self, other: Source, on: Expr) -> Self {
        self.join(other, JoinType::Right, on)
    }
    pub fn full_join(self, other: Source, on: Expr) -> Self {
        self.join(other, JoinType::Full, on)
    }
    pub fn cross_join(self, other: Source, on: Expr) -> Self {
        self.join(other, JoinType::Cross, on)
    }
}

#[cfg(test)]
#[path = "source_test.rs"]
mod tests;
