use crate::sequel::visitor::dialect::caps::{Capability, CountDistinctCap, IndexCap, IndexFormat};
use crate::sequel::visitor::dialect::{Dialect, PlaceHolderStyle};

pub struct SQLite {}
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
