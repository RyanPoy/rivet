use crate::ast::source::Source;
use crate::ast::value::Operand;

pub struct SelectStatement {
    pub select: Option<Vec<Operand>>,
    pub from: Option<Source>,
    pub _where: Option<Operand>,
}

impl SelectStatement {
    pub fn new() -> Self {
        SelectStatement { select: None, from: None, _where: None }
    }
}
