use crate::ast2::term::expr::Expr;
use crate::ast2::term::select_item::SelectItem;

#[derive(Debug, Clone)]
pub struct ColumnRef {
    pub qualifier: Option<String>, // 对应 TableRef.visible_name()
    pub name: String,
}

impl ColumnRef {
    pub fn new(name: impl Into<String>) -> Self {
        ColumnRef { qualifier: None, name: name.into() }
    }
    pub fn qualifier(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifier = Some(qualifier.into());
        self
    }
    pub fn alias(self, name: impl Into<String>) -> SelectItem {
        SelectItem::Expr { expr: Expr::Column(self), alias: Some(name.into()) }
    }
}
