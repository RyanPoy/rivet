<<<<<<< HEAD
=======
<<<<<<< HEAD:rivet-orm/src/sequel/term/column_ref.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::FuncArg;
use crate::sequel::term::literal::Literal;
use crate::sequel::term::ops::{AND, EQ, GT, GTE, IN, IS, IS_NOT, LIKE, LT, LTE, NOT_EQ, NOT_IN, NOT_LIKE, OR};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::TableInner;
<<<<<<< HEAD

=======
=======
use crate::ast2::term::expr::Expr;
use crate::ast2::term::func::FuncArg;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::ops::{AND, EQ, GT, GTE, IN, IS, IS_NOT, LIKE, LT, LTE, NOT_EQ, NOT_IN, NOT_LIKE, OR};
use crate::ast2::term::select_item::SelectItem;
use crate::ast2::term::table::TableInner;
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/column_ref.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ColumnRef {
    pub name: String,
    pub table_inner: Option<Arc<TableInner>>,
}

impl From<&str> for ColumnRef {
    fn from(value: &str) -> Self {
        Self::new(value, None)
    }
}

impl ColumnRef {
    pub fn new(name: impl Into<String>, table: Option<Arc<TableInner>>) -> Self {
        ColumnRef {
            name: name.into(),
            table_inner: table,
        }
    }
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
<<<<<<< HEAD
=======
<<<<<<< HEAD:rivet-orm/src/sequel/term/column_ref.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
        SelectItem {
            expr: Expr::Column(self),
            alias: Some(alias.into()),
        }
<<<<<<< HEAD
=======
=======
        SelectItem::Expr(Expr::Column(self), Some(alias.into()))
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/column_ref.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
    }

    pub fn distinct(self) -> FuncArg {
        Expr::Column(self).distinct()
    }

    pub fn eq<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => IS,
            _ => EQ,
        };
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op,
            right: Box::new(right),
        }
    }

    pub fn not_eq<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => IS_NOT,
            _ => NOT_EQ,
        };
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op,
            right: Box::new(right),
        }
    }

    pub fn gt<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: GT,
            right: Box::new(rhs.into()),
        }
    }

    pub fn gte<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: GTE,
            right: Box::new(rhs.into()),
        }
    }

    pub fn lt<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self.clone())),
            op: LT,
            right: Box::new(rhs.into()),
        }
    }

    pub fn lte<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: LTE,
            right: Box::new(rhs.into()),
        }
    }

    pub fn and<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: AND,
            right: Box::new(rhs.into()),
        }
    }

    pub fn or<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: OR,
            right: Box::new(rhs.into()),
        }
    }

    pub fn like<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: LIKE,
            right: Box::new(rhs.into()),
        }
    }

    pub fn not_like<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(Expr::Column(self)),
            op: NOT_LIKE,
            right: Box::new(rhs.into()),
        }
    }

    pub fn in_<T, I>(self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(Expr::Column(self)),
            list: rhs.into_iter().map(Into::into).collect(),
            negated: false,
        }
    }

    pub fn not_in<T, I>(self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(Expr::Column(self)),
            list: rhs.into_iter().map(Into::into).collect(),
            negated: true,
        }
    }
}
