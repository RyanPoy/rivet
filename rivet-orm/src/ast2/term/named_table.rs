use crate::ast2::render::SqlRender;

#[derive(Debug, Clone)]
pub struct NamedTable {
    pub name: String,
}
impl NamedTable {
    pub fn render_by(&self, render: &mut SqlRender) -> String {
        render.quote(&self.name)
    }
}
