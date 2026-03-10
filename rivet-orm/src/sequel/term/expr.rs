<<<<<<< HEAD
=======
<<<<<<< HEAD:rivet-orm/src/sequel/term/expr.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column_ref::ColumnRef;
use crate::sequel::term::func::{Func, FuncArg};
use crate::sequel::term::literal::Literal;
use crate::sequel::term::ops::{NOT, Op};
use crate::sequel::term::select_item::SelectItem;
<<<<<<< HEAD
=======
=======
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::func::{Func, FuncArg};
use crate::ast2::term::literal::Literal;
use crate::ast2::term::ops::{NOT, Op};
use crate::ast2::term::select_item::SelectItem;
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/expr.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d

#[derive(Debug, Clone)]
pub enum Expr {
    // e.g. SELECT id FROM users;
    //      SELECT u.id FROM users u;
    Column(ColumnRef),

    // e.g. SELECT 1;
    //      SELECT 'hello';
    //      SELECT true;
    //      SELECT NULL;
    Literal(Literal),

    // e.g. SELECT col in (1, 2, 3);
    //      SELECT col not in (1, 2, 3);
    In {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },

    // e.g. SELECT -price FROM orders;
    //      SELECT NOT active FROM users;
    Unary {
        op: Op,
        expr: Box<Expr>,
    },

    // e.g. SELECT price * quantity FROM orders;
    //      SELECT a + b FROM t;
    //      SELECT a = b FROM t;
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },

    // e.g. SELECT SUM(price) FROM orders;
    //      SELECT LOWER(name) FROM users;
    // Note: SELECT COUNT(*) FROM users; * is not a column ref. it's a FuncArg
    Func(Func),

    // e.g.
    // SELECT
    //     CASE
    //         WHEN price > 100 THEN 'expensive'
    //         ELSE 'cheap'
    //     END
    // FROM
    //      products;
    // Case { conditions: Vec<(Expr, Expr)>, else_expr: Option<Box<Expr>> },

    // e.g. SELECT (SELECT MAX(id) FROM users);
    Subquery(Box<SelectStatement>),
}

impl Expr {
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
<<<<<<< HEAD
=======
<<<<<<< HEAD:rivet-orm/src/sequel/term/expr.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
        SelectItem {
            expr: self,
            alias: Some(alias.into()),
        }
<<<<<<< HEAD
=======
=======
        SelectItem::Expr(self, Some(alias.into()))
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/expr.rs
>>>>>>> abcaf035f24c82033536ed8d63703aa1a1b8ef1d
    }

    pub fn distinct(self) -> FuncArg {
        FuncArg::Expr {
            expr: self,
            distinct: true,
        }
    }
}
impl From<SelectStatement> for Expr {
    fn from(stmt: SelectStatement) -> Self {
        Self::Subquery(Box::new(stmt))
    }
}
impl std::ops::Not for Expr {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Unary {
            op: NOT,
            expr: Box::new(self),
        }
    }
}

impl<T> From<T> for Expr
where
    T: Into<Literal>,
{
    fn from(value: T) -> Self {
        Expr::Literal(value.into())
    }
}

impl From<ColumnRef> for Expr {
    fn from(value: ColumnRef) -> Self {
        Expr::Column(value)
    }
}
