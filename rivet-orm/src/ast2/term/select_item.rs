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
            SelectItem::Expr {
                expr: Expr::Column(ColumnRef { name: String::from(value), qualifier: None }),
                alias: None,
            }
        }
    }
}
