use crate::sequel::statement::select::SelectStatement;
use crate::sequel::statement::select::tests::helper::{CATEGORIES, COMPANIES, ORDERS, PRODUCTS, USERS};
use crate::sequel::term::comparable::Comparable;
use crate::sequel::term::table::Table;

#[test]
fn test_cross_join() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(PRODUCTS.column("name"))
        .cross_join(&*PRODUCTS);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `products0`.`name` FROM `users` AS `users0` CROSS JOIN `products` AS `products0`",
        []
    );
}
#[test]
fn test_cross_join_with_same_table() {
    let u2 = Table::new("users");
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(u2.column("name"))
        .cross_join(u2);
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `users1`.`name` FROM `users` AS `users0` CROSS JOIN `users` AS `users1`",
        []
    );
}

#[test]
fn test_join() {
    let stmt = SelectStatement::from(&*USERS)
        .select(USERS.column("id"))
        .select(ORDERS.column("total"))
        .select(PRODUCTS.column("name"))
        .join(&*ORDERS, USERS.column("id").eq(ORDERS.column("user_id")))
        .left_join(&*PRODUCTS, ORDERS.column("id").eq(PRODUCTS.column("order_id")))
        .right_join(&*CATEGORIES, CATEGORIES.column("id").eq(PRODUCTS.column("category_id")))
        .full_join(&*COMPANIES, COMPANIES.column("id").eq(USERS.column("company_id")));
    assert_mysql!(
        &stmt,
        "SELECT `users0`.`id`, `orders0`.`total`, `products0`.`name` FROM `users` AS `users0` INNER JOIN `orders` AS `orders0` ON `users0`.`id` = `orders0`.`user_id` LEFT JOIN `products` AS `products0` ON `orders0`.`id` = `products0`.`order_id` RIGHT JOIN `categories` AS `categories0` ON `categories0`.`id` = `products0`.`category_id` FULL JOIN `companies` AS `companies0` ON `companies0`.`id` = `users0`.`company_id`",
        []
    );
}
