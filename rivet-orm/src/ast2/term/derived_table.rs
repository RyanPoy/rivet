use crate::ast2::render::SqlRender;

#[derive(Debug, Clone)]
pub struct DerivedTable {}

impl DerivedTable {
    pub fn render_by(&self, render: &mut SqlRender) -> String {
        todo!()
    }
}
