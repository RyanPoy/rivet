use crate::{model::objects::Objects, sequel::term::table::Table};

pub trait Model: Sized {
    const TABLE_NAME: &'static str;

    fn objects() -> Objects<Self> {
        Objects::new(&Self::table())
    }
    fn table_name() -> &'static str {
        Self::TABLE_NAME
    }
    fn table() -> Table {
        Table::from(Self::TABLE_NAME)
    }
}

#[cfg(test)]
#[path = "./model_test.rs"]
mod model_test;
