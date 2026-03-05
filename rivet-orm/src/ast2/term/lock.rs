use crate::ast2::term::named_table::NamedTable;

#[derive(Clone, Debug)]
pub enum Lock {
    Update,
    UpdateOf(NamedTable),
    Share,
}
#[derive(Clone, Debug)]
pub enum Wait {
    DEFAULT, // default: wait
    NoWait,
    SkipLocked,
}
