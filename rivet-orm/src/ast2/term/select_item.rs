use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;

#[derive(Clone, Debug)]
pub enum SelectItem {
    Expr { expr: Expr, alias: Option<String> },
    Wildcard,                  // SELECT * FROM users t;
    QualifiedWildcard(String), // SELECT t.* FROM users t;
}

impl From<&str> for SelectItem {
    fn from(value: &str) -> Self {
        if value == "*" {
            SelectItem::Wildcard
        } else if value.ends_with(".*") {
            SelectItem::QualifiedWildcard(value[..value.len() - 2].to_string())
        } else {
            let col = match value.split_once(".") {
                Some((q, n)) => ColumnRef { qualifier: Some(q.to_string()), name: n.to_string() },
                None => ColumnRef { qualifier: None, name: value.to_string() },
            };
            SelectItem::from(col)
        }
    }
}

impl From<ColumnRef> for SelectItem {
    fn from(col: ColumnRef) -> Self {
        SelectItem::Expr { expr: Expr::Column(col), alias: None }
    }
}