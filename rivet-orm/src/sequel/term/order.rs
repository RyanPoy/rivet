use crate::sequel::term::expr::Expr;
#[derive(Clone, Debug)]
pub enum Ordering {
    Asc,
    Desc,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub expr: Expr,
    ordering: Ordering,
}

impl Order {
    pub fn new(expr: Expr, ordering: Ordering) -> Self {
        Self { expr, ordering }
    }
    pub fn asc(expr: Expr) -> Self {
        Self {
            expr,
            ordering: Ordering::Asc,
        }
    }
    pub fn desc(expr: Expr) -> Self {
        Self {
            expr,
            ordering: Ordering::Desc,
        }
    }
    pub fn is_asc(&self) -> bool {
        match self.ordering {
            Ordering::Asc => true,
            Ordering::Desc => false,
        }
    }
    pub fn is_desc(&self) -> bool {
        !self.is_asc()
    }
}
