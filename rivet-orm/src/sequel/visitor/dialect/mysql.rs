use crate::sequel::visitor::dialect::caps::{Capability, CountDistinctCap, IndexCap, IndexFormat};
use crate::sequel::visitor::dialect::{Dialect, PlaceHolderStyle};

pub struct MySQL {}
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
