#[derive(Clone, Debug)]
pub enum Lock {
    None,
    Update,
    Share,
}
