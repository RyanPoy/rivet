use crate::ast2::sql::builder::Builder;
use crate::ast2::sql::dialect::{Dialect, LITE, MY, PG};
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::binary::Op;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::distinct::Distinct;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::join::Join;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::select_item::SelectItem;
use crate::ast2::term::subquery::Subquery;
use crate::ast2::term::table_ref::TableRef;
use std::ops::Deref;

pub struct Visitor {
    builder: Builder,
}

impl Visitor {
    pub fn new(dialect: &'static dyn Dialect) -> Self {
        Self { builder: Builder::new(dialect) }
    }
    pub fn mysql() -> Self {
        Self::new(&MY)
    }
    pub fn postgre() -> Self {
        Self::new(&PG)
    }
    pub fn sqlite() -> Self {
        Self::new(&LITE)
    }

    pub fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> &mut Self {
        self.builder.push("SELECT ");

        self.visit_distinct(&select_stmt.distinct);

        let mut iter = select_stmt.select_clause.iter();
        if let Some(item) = iter.next() {
            self.visit_select_item(item);
            for item in iter {
                self.builder.push(", ");
                self.visit_select_item(item);
            }
        } else {
            self.builder.push("*");
        }

        let mut iter = select_stmt.from_clause.iter();
        if let Some(t) = iter.next() {
            self.builder.push(" FROM ");
            self.visit_table_ref(t);
            for t in iter {
                self.builder.push(", ");
                self.visit_table_ref(t);
            }
        }

        let mut iter = select_stmt.where_clause.iter();
        if let Some(f) = iter.next() {
            self.builder.push(" WHERE ");
            self.visit_expr(f);
            for f in iter {
                self.builder.push(" AND ");
                self.visit_expr(f);
            }
        }

        self.visit_limit_and_offset(select_stmt.limit, select_stmt.offset);
        self
    }

    pub fn visit_limit_and_offset(&mut self, limit: Option<usize>, offset: Option<usize>) {
        if let Some(n) = limit {
            self.builder.push(&format!(" LIMIT {}", n));
        }
        if self.builder.dialect.supports_standalone_offset() || limit.is_some() {
            if let Some(n) = offset {
                self.builder.push(&format!(" OFFSET {}", n));
            }
        }
    }

    pub fn visit_table_ref(&mut self, table_ref: &TableRef) -> &mut Self {
        match table_ref {
            TableRef::Named { table, alias } => {
                self.visit_named_table(table);
                self.builder.push_alias(alias.as_deref());
            }
            TableRef::Subquery { subquery, alias } => {
                self.visit_subquery(subquery);
                self.builder.push_alias(Some(alias));
            }
            TableRef::Join { join, alias } => {
                self.visit_join(join);
                self.builder.push_alias(alias.as_deref());
            }
        }
        self
    }

    pub fn visit_named_table(&mut self, table: &NamedTable) -> &mut Self {
        self.builder.push_quote(table.name());
        self
    }

    pub fn visit_subquery(&mut self, subquery: &Subquery) -> &mut Self {
        self.builder.push("(");
        self.visit_select_statement(subquery.select_statement());
        self.builder.push(")");
        self
    }

    pub fn visit_join(&mut self, join: &Join) -> &mut Self {
        self
    }

    pub fn visit_select_item(&mut self, item: &SelectItem) -> &mut Self {
        match item {
            SelectItem::Wildcard => {
                self.builder.push("*");
            }
            SelectItem::QualifiedWildcard(t) => {
                self.builder.push_quote(t).push("*");
            }
            SelectItem::Expr { expr, alias } => {
                self.visit_expr(expr);
                self.builder.push_alias(alias.as_deref());
            }
        }
        self
    }

    pub fn visit_expr(&mut self, expr: &Expr) -> &mut Self {
        match expr {
            Expr::Column(c) => self.visit_column_ref(c),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Binary { left, op, right } => self.visit_expr(left).visit_op(op).visit_expr(right),
            _ => self,
        }
    }

    #[inline]
    pub fn visit_op(&mut self, op: &Op) -> &mut Self {
        self.builder.push(" ").push(op.as_ref()).push(" ");
        self
    }

    pub fn visit_distinct(&mut self, distinct: &Distinct) -> &mut Self {
        match distinct {
            Distinct::None => {}
            Distinct::Simple => {
                self.builder.push("DISTINCT ");
            }
            Distinct::On(cols) => {
                if self.builder.dialect.supports_distinct_on() {
                    self.builder.push("DISTINCT ON (");
                    let mut iter = cols.iter();
                    if let Some(item) = iter.next() {
                        self.visit_column_ref(item);
                        for item in iter {
                            self.builder.push(", ");
                            self.visit_column_ref(item);
                        }
                    }
                    self.builder.push(") ");
                } else {
                    self.builder.push("DISTINCT ");
                }
            }
        }
        self
    }

    pub fn visit_column_ref(&mut self, col: &ColumnRef) -> &mut Self {
        if let Some(q) = &col.qualifier {
            self.builder.push_quote(q).push(".");
        }
        self.builder.push_quote(&col.name);
        self
    }

    pub fn visit_literal(&mut self, lit: &Literal) -> &mut Self {
        match lit {
            Literal::Null => {
                self.builder.push("NULL");
            }
            Literal::Int(v) => {
                self.builder.push(&v.to_string());
            }
            Literal::Float(v) => {
                self.builder.push(&v.to_string());
            }
            Literal::Bool(v) => {
                if self.builder.dialect.supports_boolean() {
                    self.builder.push(&v.to_string());
                } else if *v {
                    self.builder.push("1");
                } else {
                    self.builder.push("0");
                }
            }
            Literal::String(v) => {
                let escaped = v.replace("'", "''");
                self.builder.push("'").push(&escaped).push("'");
            }
        };
        self
    }

    pub fn finish(&self) -> &str {
        &self.builder.buff
    }

    pub fn reset(&mut self) -> &mut Self {
        self.builder.clear();
        self
    }
}
