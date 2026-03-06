use crate::ast2::term::alias::Alias;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::literal::Literal;

#[derive(Clone, Debug)]
pub enum SelectItem {
    Expr { expr: Expr, alias: Option<Alias> },
    Wildcard,                  // SELECT * FROM users t;
    QualifiedWildcard(String), // SELECT t.* FROM users t;
}

impl From<&str> for SelectItem {
    fn from(value: &str) -> Self {
        if value == "*" {
            return SelectItem::Wildcard;
        }
        if value.ends_with(".*") {
            return SelectItem::QualifiedWildcard(value[..value.len() - 2].to_string());
        }

        let (name, alias) = match value.split_once(".") {
            Some((prefix, name)) => (name, Some(Alias::new(prefix.to_string()))),
            None => (value, None),
        };
        let expr = Expr::from(ColumnRef::from(name));
        SelectItem::Expr { expr, alias }
    }
}

impl From<ColumnRef> for SelectItem {
    fn from(col: ColumnRef) -> Self {
        SelectItem::Expr {
            expr: Expr::Column(col),
            alias: None,
        }
    }
}

impl From<Expr> for SelectItem {
    fn from(expr: Expr) -> Self {
        SelectItem::Expr { expr, alias: None }
    }
}

impl From<Literal> for SelectItem {
    fn from(literal: Literal) -> Self {
        SelectItem::Expr {
            expr: Expr::Literal(literal),
            alias: None,
        }
    }
}
