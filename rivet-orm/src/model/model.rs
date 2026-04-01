use crate::{
    model::objects::Objects,
    sequel::term::table::Table
};



pub trait Model: Sized {
    const TABLE_NAME: &'static str;

    fn objects()    -> Objects<Self> { Objects::new(&Self::table()) }
    fn table_name() -> &'static str  { Self::TABLE_NAME }
    fn table()      -> Table         { Table::from(Self::TABLE_NAME) }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct User { id: u64, name: String }

    impl Model for User {
        const TABLE_NAME: &'static str = "users";
    }

    #[test]
    fn test_model() {
        let _: Objects<User> = User::objects();
        assert_eq!("users", User::table_name());
    }
}
