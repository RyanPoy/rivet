#[derive(Clone, Debug)]
pub enum Lock {
    Update,
    Share,
}
#[derive(Clone, Debug)]
pub enum Wait {
    DEFAULT, // default: wait
    NoWait,
    SkipLocked,
}
