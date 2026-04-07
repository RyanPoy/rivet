use crate::sequel::term::literal::Literal;
use crate::sequel::term::ops::BinaryOp;
use crate::sequel::term::expr::Expr;


pub trait Comparable {

    fn into_expr(&self) -> Expr;

    fn eq<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => BinaryOp::Is,
            _ => BinaryOp::Eq,
        };

        Expr::Binary {
            left: Box::new(self.into_expr()),
            op,
            right: Box::new(right),
        }
    }

    fn not_eq<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        let right = rhs.into();
        let op = match right {
            Expr::Literal(Literal::Null) => BinaryOp::IsNot,
            _ => BinaryOp::NotEq,
        };

        Expr::Binary {
            left: Box::new(self.into_expr()),
            op,
            right: Box::new(right),
        }
    }

    fn gt<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Gt,
            right: Box::new(rhs.into()),
        }
    }

    fn gte<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Gte,
            right: Box::new(rhs.into()),
        }
    }

    fn lt<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Lt,
            right: Box::new(rhs.into()),
        }
    }

    fn lte<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Lte,
            right: Box::new(rhs.into()),
        }
    }

    fn like<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
        Expr::Binary {
            left: Box::new(self.into_expr()),
            op: BinaryOp::Like,
            right: Box::new(rhs.into()),
        }
    }

    fn not_like<T>(&self, rhs: T) -> Expr
    where
        T: Into<Expr>,
    {
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
            list: rhs.into_iter().map(Into::into).collect(),
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
            list: rhs.into_iter().map(Into::into).collect(),
            negated: true,
        }
    }
}
