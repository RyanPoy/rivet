use crate::sequel::build::Binder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
    pub table: Option<&'static str>,
    pub name: &'static str,
    pub alias: Option<&'static str>,
}

impl Column {
    pub fn new(name: &'static str) -> Self {
        Self { table: None, name, alias: None }
    }

    pub fn alias(mut self, name: &'static str) -> Self {
        self.alias = Some(name);
        self
    }

    pub fn table(mut self, name: &'static str) -> Self {
        self.table = Some(name);
        self
    }

    pub fn build(&self, binder: &mut Binder) -> String {
        binder.with_alias(binder.quote_full(self.table.as_deref(), self.name), self.alias.as_deref())
        // Operand::Value(v) => v.build(binder),
        // Operand::Literal(v) => v.into(),
    }
}
//
// impl IntoOperand for Column {
//     fn into_operand(binder: &mut binder) -> String {
//         Column::.build()
//     }
// }