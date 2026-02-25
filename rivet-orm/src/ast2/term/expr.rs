use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::func::FuncArg;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::select_item::SelectItem;

#[derive(Debug, Clone)]
pub enum Expr {
    // e.g. SELECT id FROM users;
    //      SELECT u.id FROM users u;
    Column(ColumnRef),

    // e.g. SELECT 1;
    //      SELECT 'hello';
    //      SELECT true;
    //      SELECT NULL;
    Literal(Literal),

    // e.g. SELECT -price FROM orders;
    //      SELECT NOT active FROM users;
    // Unary { op: UnaryOp, expr: Box<Expr> },

    // e.g. SELECT price * quantity FROM orders;
    //      SELECT a + b FROM t;
    //      SELECT a = b FROM t;
    // Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },

    // e.g. SELECT SUM(price) FROM orders;
    //      SELECT LOWER(name) FROM users;
    // Note: SELECT COUNT(*) FROM users; * is not a column ref. it's a FuncArg
    Func { name: String, args: Vec<FuncArg> },

    // e.g.
    // SELECT
    //     CASE
    //         WHEN price > 100 THEN 'expensive'
    //         ELSE 'cheap'
    //     END
    // FROM
    //      products;
    Case { conditions: Vec<(Expr, Expr)>, else_expr: Option<Box<Expr>> },

    // e.g. SELECT (SELECT MAX(id) FROM users);
    Subquery(Box<SelectStatement>),
}

impl Expr {
    pub fn alias(self, name: impl Into<String>) -> SelectItem {
        SelectItem::Expr { expr: self, alias: Some(name.into()) }
    }
}

impl From<Literal> for Expr {
    fn from(literal: Literal) -> Self {
        Expr::Literal(literal)
    }
}
