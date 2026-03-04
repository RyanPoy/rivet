use crate::ast2::sql::builder::Builder;
use crate::ast2::term::index::Index;

pub enum PlaceHolderStyle {
    QuestionMark,
    Numbered,
}

pub trait Dialect {
    fn quote_char(&self) -> &'static str;
    fn placeholder_style(&self) -> PlaceHolderStyle;

    fn supports_distinct_on(&self) -> bool;
    fn supports_window_function(&self) -> bool;
    fn supports_returning(&self) -> bool;
    fn supports_standalone_offset(&self) -> bool;
    fn supports_boolean(&self) -> bool;

    fn supports_select_for_update(&self) -> bool;

    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder);
}

pub struct MySQL;
impl Dialect for MySQL {
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

    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {
        let mut iter = indexes.iter();
        if let Some(index) = iter.next() {
            builder.push(" FORCE INDEX (").push_quote(&index.to_string());
            for index in iter {
                builder.push(", ").push_quote(&index.to_string());
            }
            builder.push(")");
        }
    }
}

pub struct PostgreSQL;
impl Dialect for PostgreSQL {
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
    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {}
}
pub struct Sqlite;
impl Dialect for Sqlite {
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

    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {
        let mut iter = indexes.iter();
        if let Some(index) = iter.next() {
            builder.push(" INDEXED BY ");
            builder.push_quote(&index.to_string());
        }
    }
}

pub static MY: MySQL = MySQL {};
pub static PG: PostgreSQL = PostgreSQL {};
pub static LITE: Sqlite = Sqlite {};
