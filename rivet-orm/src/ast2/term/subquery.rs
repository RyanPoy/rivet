use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::alias::Alias;
use crate::ast2::term::table::{TableInner, Table};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Subquery(Box<SelectStatement>);
//
// impl Deref for Subquery {
//     type Target = SelectStatement;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl From<SelectStatement> for Subquery {
    fn from(statement: SelectStatement) -> Self {
        Subquery(Box::new(statement))
    }
}

impl Subquery {
    pub fn alias(self, alias: impl Into<Alias>) -> Table {
        let inner = TableInner::Subquery(self);

        Table {
            inner: Arc::new(inner),
            alias: Some(alias.into()),
        }
    }

    #[inline]
    pub fn select_statement(&self) -> &SelectStatement {
        &self.0
    }
}
