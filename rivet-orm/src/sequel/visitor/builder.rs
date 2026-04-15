use crate::sequel::term::param::{Param, ParamData};
use crate::sequel::visitor::dialect::{Dialect, PlaceHolderStyle};

pub struct Builder {
    pub buff: String,
    pub binder: Vec<ParamData>,
}

impl Builder {
    pub fn new() -> Self {
        Self::with_capacity(512)
    }
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buff: String::with_capacity(size),
            binder: Vec::new(),
        }
    }
    #[inline]
    pub fn push(&mut self, s: &str) -> &mut Self {
        self.buff.push_str(s);
        self
    }

    pub fn bind(&mut self, value: ParamData, dialect: &impl Dialect) -> &mut Self {
        self.binder.push(value);
        match dialect.placeholder_style() {
            PlaceHolderStyle::QuestionMark => self.buff.push_str("?"),
            PlaceHolderStyle::Numbered => {
                self.buff.push_str("$");
                self.buff.push_str(&self.binder.len().to_string());
            },
        }
        self
    }
}
