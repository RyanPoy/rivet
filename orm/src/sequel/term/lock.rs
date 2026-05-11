use crate::sequel::term::table::Table;

#[derive(Clone, Debug)]
pub enum Lock {
    Update,
    UpdateOf(Vec<Table>),
    Share,
}

#[derive(Clone, Debug)]
pub enum Wait {
    Default, // default: wait
    NoWait,
    SkipLocked,
}

#[derive(Clone, Debug)]
pub struct Locking {
    pub lock: Option<Lock>,
    pub wait: Option<Wait>,
}

impl Locking {
    pub fn new() -> Locking {
        Self { lock: None, wait: None }
    }
}
