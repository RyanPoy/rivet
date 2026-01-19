use crate::ast::sql_value::ToSql;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: &'static str,
        op: &'static str,
        right: Box<dyn ToSql>,
    },
}


#[cfg(test)]
#[path = "expression_test.rs"]
mod tests;