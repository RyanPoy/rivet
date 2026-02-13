use crate::ast2::term::named_column::NamedColumn;

pub enum Expr {
    Column(NamedColumn),
}
