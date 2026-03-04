use crate::ast2::sql::dialect::{Dialect, PlaceHolderStyle};
use crate::ast2::term::literal::Literal;

pub struct Builder {
    pub dialect: &'static dyn Dialect,
    pub buff: String,
    pub binder: Vec<Literal>,
}

impl Builder {
    pub fn new(dialect: &'static dyn Dialect) -> Self {
        Self::with_capacity(dialect, 512)
    }
    pub fn with_capacity(dialect: &'static dyn Dialect, size: usize) -> Self {
        Self {
            dialect,
            buff: String::with_capacity(size),
            binder: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, s: &str) -> &mut Self {
        self.buff.push_str(s);
        self
    }

    pub fn bind(&mut self, value: Literal) -> &mut Self {
        self.binder.push(value);
        match self.dialect.placeholder_style() {
            PlaceHolderStyle::QuestionMark => self.buff.push_str("?"),
            PlaceHolderStyle::Numbered => self.buff.push_str(&format!("${}", self.binder.len())),
        }
        self
    }

    pub fn push_quote(&mut self, s: &str) -> &mut Self {
        let quote_char = self.dialect.quote_char();
        self.push(quote_char).push(s).push(quote_char)
    }

    pub fn clear(&mut self) -> &mut Self {
        self.buff.clear();
        self.binder.clear();
        self
    }
}
