use crate::sequel::ast::Expr;
use crate::sequel::ast::SelectStatement;
use crate::sequel::build::Binder;
#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct Table {
    pub schema: Option<&'static str>,
    pub name: &'static str,
    pub alias: Option<&'static str>,
}
impl Table {
    pub fn new(name: &'static str) -> Self {
        Self { schema: None, name, alias: None }
    }
    pub fn schema(mut self, name: &'static str) -> Self {
        self.schema = Some(name);
        self
    }
    pub fn alias(mut self, name: &'static str) -> Self {
        self.alias = Some(name);
        self
    }
}

#[derive(Clone)]
pub enum Source {
    Table(Table),
    SubQuery { query: Box<SelectStatement>, alias: Option<&'static str> },
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
impl JoinType {
    pub fn build(&self, _: &mut Binder) -> String {
        match self {
            Self::Inner => "INNER JOIN".to_string(),
            Self::Left => "LEFT JOIN".to_string(),
            Self::Right => "RIGHT JOIN".to_string(),
            Self::Full => "FULL JOIN".to_string(),
            Self::Cross => "CROSS JOIN".to_string(),
        }
    }
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
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Source::Table(Table { schema, name, alias }) => {
                let sql = binder.quote_full(schema.as_deref(), name);
                binder.with_alias(sql, alias.as_deref())
            }
            Source::SubQuery { query, alias } => {
                let sql = format!("({})", query.build(binder));
                binder.with_alias(sql, alias.as_deref())
            }
            Source::Join { left: _, right, tp, on } => {
                format!("{} {} ON {}", right.build(binder), tp.build(binder), on.build(binder))
            }
        }
    }
}

#[cfg(test)]
#[path = "source_test.rs"]
mod tests;
