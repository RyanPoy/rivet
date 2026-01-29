use crate::sequel::ast::Expr;
use crate::sequel::ast::SelectStatement;
use crate::sequel::build::Binder;

#[derive(Clone)]
pub enum Source {
    Table { name: &'static str, alias: Option<&'static str> },
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
            Source::Table { name, alias } => match alias {
                Some(alias_name) => format!("{} AS {}", binder.quote(name), binder.quote(alias_name)),
                None => binder.quote(name),
            },
            Source::SubQuery { query, alias } => match alias {
                Some(alias_name) => format!("{} AS {}", query.build(binder), alias_name),
                None => query.build(binder),
            },
            Source::Join { left: _, right, tp, on } => {
                format!("{} {} ON {}", right.build(binder), tp.build(binder), on.build(binder))
            }
        }
    }
}

#[cfg(test)]
#[path = "source_test.rs"]
mod tests;
