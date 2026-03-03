pub enum PlaceHolderStyle {
    QuestionMark,
    Numbered,
}

pub trait Dialect {
    fn name(&self) -> &'static str;
    fn quote_char(&self) -> &'static str;
    fn placeholder_style(&self) -> PlaceHolderStyle;

    fn supports_distinct_on(&self) -> bool;
    fn supports_window_function(&self) -> bool;
    fn supports_returning(&self) -> bool;
    fn supports_standalone_offset(&self) -> bool;
    fn supports_boolean(&self) -> bool;

    fn supports_select_for_update(&self) -> bool;
}

pub struct MySQL;
impl Dialect for MySQL {
    #[inline]
    fn name(&self) -> &'static str {
        "MySQL"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "`"
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::QuestionMark
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        false
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        true // 8+
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        false
    }
    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        false
    }
    #[inline]
    fn supports_boolean(&self) -> bool {
        false
    }
    #[inline]
    fn supports_select_for_update(&self) -> bool {
        true
    }
}

pub struct PostgreSQL;
impl Dialect for PostgreSQL {
    #[inline]
    fn name(&self) -> &'static str {
        "PostgreSQL"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::Numbered
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        true
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        true
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        true
    }
    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        true
    }
    #[inline]
    fn supports_boolean(&self) -> bool {
        true
    }
    #[inline]
    fn supports_select_for_update(&self) -> bool {
        true
    }
}
pub struct Sqlite;
impl Dialect for Sqlite {
    #[inline]
    fn name(&self) -> &'static str {
        "Sqlite"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::QuestionMark
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        false
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        false
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        true
    }
    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        true
    }
    #[inline]
    fn supports_boolean(&self) -> bool {
        false
    }
    #[inline]
    fn supports_select_for_update(&self) -> bool {
        false
    }
}

pub static MY: MySQL = MySQL {};
pub static PG: PostgreSQL = PostgreSQL {};
pub static LITE: Sqlite = Sqlite {};
