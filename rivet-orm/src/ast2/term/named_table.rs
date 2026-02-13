#[derive(Debug, Clone)]
pub struct NamedTable {
    pub name: String,
    pub alias: Option<String>,
}
impl NamedTable {
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
}
