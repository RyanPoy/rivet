use crate::sequel::ast::Operand;
use crate::sequel::build::Binder;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub column: Operand,
    pub direction: Direction,
}

impl Order {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self.direction {
            Direction::Asc => format!("{} ASC", self.column.build(binder)),
            Direction::Desc => format!("{} DESC", self.column.build(binder)),
        }
    }
}
