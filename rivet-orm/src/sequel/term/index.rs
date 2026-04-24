use rivet_utils::impl_into_vec_for;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Index(String);
impl_into_vec_for!(Index => [Index, &'static str]);

impl<T> From<T> for Index
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Indexes {
    pub force: Vec<Index>,
    pub use_: Vec<Index>,
    pub ignore: Vec<Index>,
}

impl Indexes {
    pub fn new() -> Self {
        Self {
            force: Vec::new(),
            use_: Vec::new(),
            ignore: Vec::new(),
        }
    }

    pub fn push_force(&mut self, index: Index) {
        self.force.push(index);
    }

    pub fn push_use(&mut self, index: Index) {
        self.use_.push(index);
    }

    pub fn push_ignore(&mut self, index: Index) {
        self.ignore.push(index);
    }
}
