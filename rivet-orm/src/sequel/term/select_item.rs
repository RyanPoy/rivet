use crate::sequel::term::column::Column;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::literal::Literal;

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub expr: Expr,
    pub alias: Option<String>,
}

impl From<Column> for SelectItem {
    fn from(value: Column) -> Self {
        Self {
            expr: Expr::Column(value),
            alias: None,
        }
    }
}
impl From<Literal> for SelectItem {
    fn from(value: Literal) -> Self {
        Self {
            expr: Expr::Literal(value),
            alias: None,
        }
    }
}
impl From<Expr> for SelectItem {
    fn from(expr: Expr) -> Self {
        Self { expr, alias: None }
    }
}

pub trait IntoSelectItems {
    fn into_select_items(self) -> Vec<SelectItem>;
}

impl<T> IntoSelectItems for T
where
    T: Into<SelectItem>,
{
    fn into_select_items(self) -> Vec<SelectItem> {
        vec![self.into()]
    }
}

impl<T> IntoSelectItems for Vec<T>
where
    T: Into<SelectItem>,
{
    fn into_select_items(self) -> Vec<SelectItem> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T, const N: usize> IntoSelectItems for [T; N]
where
    T: Into<SelectItem>,
{
    fn into_select_items(self) -> Vec<SelectItem> {
        self.into_iter().map(Into::into).collect()
    }
}
