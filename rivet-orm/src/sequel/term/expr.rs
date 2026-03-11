use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::func::{Func, FuncArg};
use crate::sequel::term::literal::Literal;
use crate::sequel::term::ops::{BinaryOp, UnaryOp};
use crate::sequel::term::select_item::SelectItem;

#[derive(Debug, Clone)]
pub enum Expr {
    // e.g. SELECT id FROM users;
    //      SELECT u.id FROM users u;
    Column(Column),

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
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // e.g. SELECT price * quantity FROM orders;
    //      SELECT a + b FROM t;
    //      SELECT a = b FROM t;
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
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
        SelectItem {
            expr: self,
            alias: Some(alias.into()),
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Expr::Binary { op, .. } => op.precedence(),
            Expr::Unary { .. } => 25,
            _ => 100,
        }
    }

    pub fn distinct(self) -> FuncArg {
        FuncArg::Expr {
            expr: self,
            distinct: true,
        }
    }

    pub fn and<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self),
            op: BinaryOp::And,
            right: Box::new(rhs.into()),
        }
    }

    pub fn or<T>(self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self),
            op: BinaryOp::Or,
            right: Box::new(rhs.into()),
        }
    }
}
impl<T> From<Option<T>> for Expr
where
    T: Into<Expr>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Expr::Literal(Literal::Null),
        }
    }
}
impl From<SelectStatement> for Expr {
    fn from(stmt: SelectStatement) -> Self {
        Self::Subquery(Box::new(stmt))
    }
}
impl From<Column> for Expr {
    fn from(value: Column) -> Self {
        Expr::Column(value)
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
impl std::ops::Not for Expr {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Unary {
            op: UnaryOp::Not,
            expr: Box::new(self),
        }
    }
}
