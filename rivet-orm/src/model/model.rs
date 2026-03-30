
pub trait Model: Sized{
    type Objects;

    fn objects() -> Self::Objects;
}

#[cfg(test)]
mod tests {
    use crate::{model::objects::Objects, sequel::term::table::Table};

    use super::*;

    struct User { id: u64, name: String }
    struct UserObjects { inner: Objects<User> }

    impl Model for User {
        type Objects = UserObjects;


        fn objects() -> Self::Objects {
           const TABLE_NAME: &str = "users";
           let t = Table::from(TABLE_NAME);
           UserObjects { inner: Objects::new(&t) }
        }
    }

    #[test]
    fn test_model_objects() {
        let users = User::objects();
    }
}
