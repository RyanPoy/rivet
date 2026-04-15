use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::param::Param;
use crate::sequel::term::ops::BinaryOp;

macro_rules! impl_arithmetic {
    ($($op:ident => $fn_name:ident),*) => {
        $(
            impl std::ops::$op<Column> for Column {
                type Output = Expr;
                fn $fn_name(self, rhs: Column) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }

            impl std::ops::$op<Param> for Column {
                type Output = Expr;
                fn $fn_name(self, rhs: Param) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }

            impl std::ops::$op<Param> for Param {
                type Output = Expr;
                fn $fn_name(self, rhs: Param) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }

            impl std::ops::$op<Column> for Param {
                type Output = Expr;
                fn $fn_name(self, rhs: Column) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }

            impl std::ops::$op<Param> for Expr {
                type Output = Expr;
                fn $fn_name(self, rhs: Param) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }

            impl std::ops::$op<Column> for Expr {
                type Output = Expr;
                fn $fn_name(self, rhs: Column) -> Self::Output {
                    Expr::Binary { left: Box::new(self.into()), op: BinaryOp::$op, right: Box::new(rhs.into()), }
                }
            }
        )*
    };
}

impl_arithmetic!(Add => add, Sub => sub, Mul => mul, Div => div, Rem => rem);
