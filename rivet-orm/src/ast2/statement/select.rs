use crate::ast2::term::table_ref::TableRef;

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub select_clause: Vec<String>,
    pub from_clause: Vec<TableRef>,
}

impl SelectStatement {
    pub fn new() -> Self {
        Self { select_clause: Vec::new(), from_clause: Vec::new() }
    }

    pub fn from<T>(mut self, t: T) -> Self
    where
        T: Into<TableRef>,
    {
        self.from_clause.push(t.into());
        self
    }

    pub fn from_many<T, I>(mut self, ts: I) -> Self
    where
        T: Into<TableRef>,
        I: IntoIterator<Item = T>,
    {
        for t in ts {
            self.from_clause.push(t.into());
        }
        self
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
