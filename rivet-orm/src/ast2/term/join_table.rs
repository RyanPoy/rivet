use crate::ast2::render::SqlRender;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl JoinType {
    pub fn render_by(render: SqlRender) -> String {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct JoinedTable {}
impl JoinedTable {
    pub fn render_by(&self, render: &mut SqlRender) -> String {
        todo!()
    }
}
