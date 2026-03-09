use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::literal::Literal;

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub expr: Expr,
    pub alias: Option<String>,
}

macro_rules! impl_from_for_select_item {
    ($($t:ty => $variant:ident), *) => {
        $(
            impl From<$t> for SelectItem {
                fn from(value: $t) -> Self {
                    SelectItem{expr: Expr::$variant(value), alias: None}
                }
            }
            impl From<&$t> for SelectItem {
                fn from(value: &$t) -> Self {
                    SelectItem{expr: Expr::$variant(value.clone()), alias: None}
                }
            }
        )*
    };
}

impl_from_for_select_item!(
    ColumnRef => Column,
    Literal => Literal
);

impl From<&str> for SelectItem {
    fn from(value: &str) -> Self {
        let (name, alias) = match value.split_once(".") {
            Some((prefix, name)) => (name, Some(prefix.to_string())),
            None => (value, None),
        };
        let expr = Expr::from(ColumnRef::from(name));
        SelectItem { expr, alias }
    }
}

impl From<Expr> for SelectItem {
    fn from(expr: Expr) -> Self {
        SelectItem { expr, alias: None }
    }
}

impl From<&Expr> for SelectItem {
    fn from(expr: &Expr) -> Self {
        SelectItem {
            expr: expr.clone(),
            alias: None,
        }
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
