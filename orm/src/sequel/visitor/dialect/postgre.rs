use crate::sequel::visitor::dialect::caps::{Capability, CountDistinctCap};
use crate::sequel::visitor::dialect::{Dialect, PlaceHolderStyle};

pub struct PostgreSQL {}
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
