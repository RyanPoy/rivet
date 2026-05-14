use crate::model::columns::ColumnType;

#[derive(Debug, Clone)]
pub struct CharColumn {
    pub name: &'static str,
    pub column_type: ColumnType,
    pub length: usize,
}

impl CharColumn {
    pub const fn new(name: &'static str, length: usize) -> Self {
        Self {
            name,
            column_type: ColumnType::Char { max_length: Some(length) },
            length,
        }
    }
}
