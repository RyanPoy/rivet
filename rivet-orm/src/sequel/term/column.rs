use crate::sequel::term::expr::Expr;
use crate::sequel::term::literal::Literal;
use crate::sequel::term::ops::BinaryOp;
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::TableInner;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub table_inner: Arc<TableInner>,
}

impl Column {
    pub fn new(name: impl Into<String>, table: Arc<TableInner>) -> Self {
        Column {
            name: name.into(),
            table_inner: table,
        }
    }
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        SelectItem {
            expr: Expr::Column(self),
            alias: Some(alias.into()),
        }
    }

    pub fn eq<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => BinaryOp::Is,
            _ => BinaryOp::Eq,
        };
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op,
            right: Box::new(right),
        }
    }

    pub fn not_eq<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => BinaryOp::IsNot,
            _ => BinaryOp::NotEq,
        };
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op,
            right: Box::new(right),
        }
    }

    pub fn gt<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::Gt,
            right: Box::new(rhs.into()),
        }
    }

    pub fn gte<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::Gte,
            right: Box::new(rhs.into()),
        }
    }

    pub fn lt<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::Lt,
            right: Box::new(rhs.into()),
        }
    }

    pub fn lte<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::Lte,
            right: Box::new(rhs.into()),
        }
    }

    pub fn like<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::Like,
            right: Box::new(rhs.into()),
        }
    }

    pub fn not_like<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: BinaryOp::NotLike,
            right: Box::new(rhs.into()),
        }
    }

    pub fn in_<T, I>(&self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(Expr::Column(self.clone())),
            list: rhs.into_iter().map(Into::into).collect(),
            negated: false,
        }
    }

    pub fn not_in<T, I>(&self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(Expr::Column(self.clone())),
            list: rhs.into_iter().map(Into::into).collect(),
            negated: true,
        }
    }
}
