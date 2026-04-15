use crate::sequel::term::expr::Expr;
use crate::sequel::term::ops::BinaryOp;
use crate::sequel::term::param::Param;

pub trait Comparable {
    fn into_expr(&self) -> Expr;

    fn eq(&self, rhs: impl Into<Expr>) -> Expr {
        let right = rhs.into();
        let op = match &right {
            Expr::Param(Param::Null) => BinaryOp::Is,
            _ => BinaryOp::Eq,
        };

        Expr::Binary {
            left: Box::new(self.into_expr()),
            op,
            right: Box::new(right),
        }
    }

    fn not_eq(&self, rhs: impl Into<Expr>) -> Expr {
        let right = rhs.into();
        let op = match &right {
            Expr::Param(Param::Null) => BinaryOp::IsNot,
            _ => BinaryOp::NotEq,
        };

        Expr::Binary {
            left: Box::new(self.into_expr()),
            op,
            right: Box::new(right),
        }
    }

    fn gt(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Gt,
            right: Box::new(rhs.into()),
        }
    }

    fn gte(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Gte,
            right: Box::new(rhs.into()),
        }
    }

    fn lt(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Lt,
            right: Box::new(rhs.into()),
        }
    }

    fn lte(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Lte,
            right: Box::new(rhs.into()),
        }
    }

    fn like(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Like,
            right: Box::new(rhs.into()),
        }
    }

    fn not_like(&self, rhs: impl Into<Expr>) -> Expr {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::NotLike,
            right: Box::new(rhs.into()),
        }
    }

    fn in_<T, I>(&self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(self.into_expr()),
            list: rhs.into_iter().map(|e| e.into()).collect(),
            negated: false,
        }
    }

    fn not_in<T, I>(&self, rhs: I) -> Expr
    where
        T: Into<Expr>,
        I: IntoIterator<Item = T>,
    {
        Expr::In {
            expr: Box::new(self.into_expr()),
            list: rhs.into_iter().map(|e| e.into()).collect(),
            negated: true,
        }
    }
}