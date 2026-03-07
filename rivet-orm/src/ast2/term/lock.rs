#[derive(Clone, Debug)]
pub enum Lock {
    Update,
    UpdateOf(String),
    Share,
}
#[derive(Clone, Debug)]
pub enum Wait {
    DEFAULT, // default: wait
    NoWait,
    SkipLocked,
}
