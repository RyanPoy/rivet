pub mod caps;
pub mod mysql;
pub mod postgre;
pub mod sqlite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceHolderStyle {
    QuestionMark,
    Numbered,
}
pub trait Dialect {
    fn caps(&self) -> caps::Capability;
    fn quote_char(&self) -> &'static str;
    fn placeholder_style(&self) -> PlaceHolderStyle;
    fn bool_str(&self, v: bool) -> &'static str;
}
