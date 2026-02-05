use crate::ast2::render::SqlRender;
use crate::ast2::term::derived_table::DerivedTable;
use crate::ast2::term::join_table::JoinedTable;
use crate::ast2::term::named_table::NamedTable;

#[derive(Debug, Clone)]
pub enum TableRef {
    NamedTable(NamedTable),
    DerivedTable(DerivedTable),
    JoinedTable(JoinedTable),
}

impl From<&str> for TableRef {
    fn from(value: &str) -> Self {
        Self::NamedTable(NamedTable { name: value.to_string() })
    }
}
impl From<NamedTable> for TableRef {
    fn from(value: NamedTable) -> Self {
        Self::NamedTable(value)
    }
}

impl TableRef {
    pub fn render_by(&self, render: &mut SqlRender) -> String {
        match self {
            Self::DerivedTable(t) => t.render_by(render),
            Self::NamedTable(t) => t.render_by(render),
            Self::JoinedTable(t) => t.render_by(render),
        }
    }
}
