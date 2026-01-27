use crate::ast::expr::Expr;
use crate::ast::source::Source;
use crate::ast::value::Operand;

pub struct SelectStatement {
    pub select: Vec<Operand>,
    pub from: Option<Source>,
    pub _where: Option<Expr>,
}

impl SelectStatement {
    pub fn new() -> Self {
        SelectStatement { select: vec![], from: None, _where: None }
    }
}
