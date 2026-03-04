use crate::ast2::sql::builder::Builder;
use crate::ast2::term::index::Index;
use std::sync::LazyLock;

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

    fn supports_select_for_update(&self) -> bool;
    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder);
    fn bool_str(&self, v: bool) -> &'static str;
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
    fn supports_select_for_update(&self) -> bool {
        true
    }

    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {
        let mut iter = indexes.iter();
        if let Some(index) = iter.next() {
            let char = self.quote_char();
            builder
                .push(" FORCE INDEX (")
                .push(char)
                .push(&index.to_string())
                .push(char);
            for index in iter {
                builder.push(", ").push(char).push(&index.to_string()).push(char);
            }
            builder.push(")");
        }
    }
    #[inline]
    fn bool_str(&self, v: bool) -> &'static str {
        if v { "1" } else { "0" }
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
    fn supports_select_for_update(&self) -> bool {
        true
    }
    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {}
    #[inline]
    fn bool_str(&self, v: bool) -> &'static str {
        if v { "true" } else { "false" }
    }
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
    fn supports_select_for_update(&self) -> bool {
        false
    }

    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {
        let mut iter = indexes.iter();
        if let Some(index) = iter.next() {
            let char = self.quote_char();
            builder
                .push(" INDEXED BY ")
                .push(char)
                .push(&index.to_string())
                .push(char);
        }
    }
    #[inline]
    fn bool_str(&self, v: bool) -> &'static str {
        if v { "1" } else { "0" }
    }
}
