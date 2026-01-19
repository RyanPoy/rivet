use crate::ast::expression::Expr;
use crate::ast::sql_value::SqlValue;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Column<T> {
    pub name: &'static str,
    _marker: PhantomData<T>,
}

impl<T> Column<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }

    pub fn eq<V: SqlValue<T> + 'static>(&self, v: V) -> Expr {
        Expr::Binary {
            left: self.name,
            op: v.binary_op_eq(),
            right: Box::new(v),
        }
    }
}

#[cfg(test)]
#[path = "schema_test.rs"]
mod tests; // 告诉编译器，这个模块的内容在 a_test.rs 里
