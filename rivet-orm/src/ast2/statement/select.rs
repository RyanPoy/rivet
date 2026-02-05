use crate::ast2::render::SqlRender;
use crate::ast2::term::table_ref::TableRef;

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

    pub fn render_by(&self, render: &mut SqlRender) -> String {
        let mut parts = Vec::<String>::new();

        parts.push(self.render_select(render));
        parts.push(self.render_from(render));

        parts.join(" ")
    }

    fn render_select(&self, render: &mut SqlRender) -> String {
        "SELECT *".to_string()
    }

    fn render_from(&self, render: &mut SqlRender) -> String {
        let sql: Vec<String> = self.from_clause.iter().map(|t| t.render_by(render)).collect();
        format!("FROM {}", sql.join(", "))
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
