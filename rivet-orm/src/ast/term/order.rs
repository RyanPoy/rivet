use crate::ast::Operand;
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
