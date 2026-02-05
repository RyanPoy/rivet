#[derive(Debug, Clone)]
pub struct ColumnRef {
    pub table: Option<String>, // 对应 TableRef.visible_name()
    pub name: String,
}
