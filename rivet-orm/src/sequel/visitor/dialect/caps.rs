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
