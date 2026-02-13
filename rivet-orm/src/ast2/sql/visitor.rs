use crate::ast2::sql::builder::Builder;
use crate::ast2::sql::dialect::Dialect;
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::derived_table::DerivedTable;
use crate::ast2::term::join_table::JoinedTable;
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::table_ref::TableRef;

pub struct Visitor {
    builder: Builder,
}

impl Visitor {
    pub fn new(dialect: Dialect) -> Self {
        Self { builder: Builder::new(dialect) }
    }
    pub fn mysql() -> Self {
        Self::new(Dialect::MySQL)
    }
    pub fn postgre() -> Self {
        Self::new(Dialect::PostgreSQL)
    }
    pub fn sqlite() -> Self {
        Self::new(Dialect::SQLite)
    }

    pub fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> &mut Self {
        self.builder.push("SELECT *");

        self.builder.push(" FROM ");
        let mut iter = select_stmt.from_clause.iter();
        if let Some(t) = iter.next() {
            self.visit_table_ref(t);
        }
        for t in iter {
            self.builder.push(", ");
            self.visit_table_ref(t);
        }
        self
    }
    pub fn visit_table_ref(&mut self, table_ref: &TableRef) -> &mut Self {
        match table_ref {
            TableRef::NamedTable(table) => self.visit_named_table(table),
            TableRef::DerivedTable(table) => self.visit_derived_table(table),
            TableRef::JoinedTable(table) => self.visit_joined_table(table),
        }
    }
    pub fn visit_named_table(&mut self, table: &NamedTable) -> &mut Self {
        self.builder.push_with_alias(&table.name, table.alias.as_deref());
        self
    }

    pub fn visit_derived_table(&mut self, table: &DerivedTable) -> &mut Self {
        self.builder.push("(");
        self.visit_select_statement(&table.stmt);
        self.builder.push(")");
        self.builder.push(" AS ");
        self.builder.push_quote(table.alias.as_deref().unwrap());
        self
    }

    pub fn visit_joined_table(&mut self, table: &JoinedTable) -> &mut Self {
        self
    }

    pub fn finish(&self) -> &str {
        &self.builder.buff
    }
}
