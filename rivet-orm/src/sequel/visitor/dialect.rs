use crate::sequel::term::index::Index;
use crate::sequel::visitor::builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceHolderStyle {
    QuestionMark,
    Numbered,
}
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub enum CountDistinctCap {
    #[default]
    Extend,
    Merge,
    Rewrite,
}
#[derive(Clone, Debug, Copy, Default)]
pub struct Capability {
    pub distinct_on: bool,
    pub returning: bool,
    pub standalone_offset: bool,
    pub select_with_locking: bool,
    pub count_distinct: CountDistinctCap,
}
impl Capability {
    pub fn all() -> Self {
        Capability {
            distinct_on: true,
            returning: true,
            standalone_offset: true,
            select_with_locking: true,
            count_distinct: CountDistinctCap::default(),
        }
    }
}

pub trait Dialect {
    fn caps(&self) -> Capability;
    fn quote_char(&self) -> &'static str;
    fn placeholder_style(&self) -> PlaceHolderStyle;
    fn bool_str(&self, v: bool) -> &'static str;
    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder);
}

pub struct MySQL;
impl Dialect for MySQL {
    #[inline]
    fn caps(&self) -> Capability {
        Capability {
            count_distinct: CountDistinctCap::Extend,
            select_with_locking: true,
            ..Capability::default()
        }
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "`"
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::QuestionMark
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
    fn caps(&self) -> Capability {
        Capability {
            count_distinct: CountDistinctCap::Merge,
            ..Capability::all()
        }
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::Numbered
    }
    fn render_force_index_hint(&self, indexes: &[Index], builder: &mut Builder) {}
    #[inline]
    fn bool_str(&self, v: bool) -> &'static str {
        if v { "true" } else { "false" }
    }
}

pub struct SQLite;
impl Dialect for SQLite {
    #[inline]
    fn caps(&self) -> Capability {
        Capability {
            returning: true,
            standalone_offset: true,
            count_distinct: CountDistinctCap::Rewrite,
            ..Capability::default()
        }
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn placeholder_style(&self) -> PlaceHolderStyle {
        PlaceHolderStyle::QuestionMark
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
