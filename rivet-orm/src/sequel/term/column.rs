use crate::sequel::term::comparable::Comparable;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::TableInner;
use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub table_inner: Arc<TableInner>,
}

impl Column {
    pub fn new(name: impl Into<String>, table: Arc<TableInner>) -> Self {
        Column {
            name: name.into(),
            table_inner: table,
        }
    }
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        SelectItem {
            expr: Expr::Column(self),
            alias: Some(alias.into()),
        }
    }

}

impl Comparable for Column {
   fn into_expr(&self) -> Expr { Expr::Column(self.clone()) }
}
