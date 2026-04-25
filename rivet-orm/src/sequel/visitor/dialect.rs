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
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub struct IndexFormat {
    pub before: &'static str,
    pub after: &'static str,
    pub support_multiple: bool,
}

#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub struct IndexCap {
    pub force: Option<IndexFormat>,
    pub use_: Option<IndexFormat>,
    pub ignore: Option<IndexFormat>,
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Capability {
    pub distinct_on: bool,
    pub returning: bool,
    pub standalone_offset: bool,
    pub select_with_locking: bool,
    pub count_distinct: CountDistinctCap,
    pub index_cap: IndexCap,
}

impl Capability {
    pub fn all() -> Self {
        Capability {
            distinct_on: true,
            returning: true,
            standalone_offset: true,
            select_with_locking: true,
            count_distinct: CountDistinctCap::default(),
            index_cap: IndexCap {
                force: None,
                use_: None,
                ignore: None,
            },
        }
    }
}

pub trait Dialect {
    fn caps(&self) -> Capability;
    fn quote_char(&self) -> &'static str;
    fn placeholder_style(&self) -> PlaceHolderStyle;
    fn bool_str(&self, v: bool) -> &'static str;
}

pub struct MySQL;
impl Dialect for MySQL {
    #[inline]
    fn caps(&self) -> Capability {
        Capability {
            count_distinct: CountDistinctCap::Extend,
            select_with_locking: true,
            index_cap: IndexCap {
                force: Some(IndexFormat {
                    before: "FORCE INDEX (",
                    after: ")",
                    support_multiple: true,
                }),
                use_: Some(IndexFormat {
                    before: "USE INDEX (",
                    after: ")",
                    support_multiple: true,
                }),
                ignore: Some(IndexFormat {
                    before: "IGNORE INDEX (",
                    after: ")",
                    support_multiple: true,
                }),
            },
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
            index_cap: IndexCap {
                force: Some(IndexFormat {
                    before: "INDEXED BY",
                    after: "",
                    support_multiple: false,
                }),
                use_: None,
                ignore: Some(IndexFormat {
                    before: "IGNORE INDEX (",
                    after: ")",
                    support_multiple: false,
                }),
            },
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

    #[inline]
    fn bool_str(&self, v: bool) -> &'static str {
        if v { "1" } else { "0" }
    }
}
