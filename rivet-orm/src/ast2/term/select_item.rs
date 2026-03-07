use crate::ast2::term::alias::Alias;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::table::TableInner;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum SelectItem {
    Expr(Expr, Option<Alias>),
    All(Option<Arc<TableInner>>),
}

macro_rules! impl_from_for_select_item {
    ($($t:ty => $variant:ident), *) => {
        $(
            impl From<$t> for SelectItem {
                fn from(value: $t) -> Self {
                    SelectItem::Expr(Expr::$variant(value), None)
                }
            }
            impl From<&$t> for SelectItem {
                fn from(value: &$t) -> Self {
                    SelectItem::Expr(Expr::$variant(value.clone()), None)
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
        if value == "*" {
            return SelectItem::All(None);
        }
        let (name, alias) = match value.split_once(".") {
            Some((prefix, name)) => (name, Some(Alias::new(prefix.to_string()))),
            None => (value, None),
        };
        let expr = Expr::from(ColumnRef::from(name));
        SelectItem::Expr(expr, alias)
    }
}

impl From<Expr> for SelectItem {
    fn from(expr: Expr) -> Self {
        SelectItem::Expr(expr, None)
    }
}

impl From<&Expr> for SelectItem {
    fn from(expr: &Expr) -> Self {
        SelectItem::Expr(expr.clone(), None)
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
