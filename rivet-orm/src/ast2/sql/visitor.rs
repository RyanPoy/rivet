use crate::ast2::sql::builder::Builder;
use crate::ast2::sql::dialect::{Dialect, MySQL, PostgreSQL, SQLite};
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::alias::Alias;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::distinct::Distinct;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::index::Index;
use crate::ast2::term::join::Join;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::lock::{Lock, Wait};
use crate::ast2::term::named_table::NamedTable;
use crate::ast2::term::ops::{IN, NOT_IN, Op};
use crate::ast2::term::select_item::SelectItem;
use crate::ast2::term::subquery::Subquery;
use crate::ast2::term::table_ref::TableRef;

pub fn mysql() -> Visitor<MySQL> {
    Visitor::new(MySQL {})
}

pub fn postgre() -> Visitor<PostgreSQL> {
    Visitor::new(PostgreSQL {})
}

pub fn sqlite() -> Visitor<SQLite> {
    Visitor::new(SQLite {})
}
pub struct Visitor<D> {
    builder: Builder,
    dialect: D,
}

impl<D: Dialect> Visitor<D> {
    pub fn new(dialect: D) -> Self {
        Self {
            builder: Builder::new(),
            dialect,
        }
    }

    pub fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> &mut Self {
        self.push("SELECT ");
        self.visit_distinct(&select_stmt.distinct);
        self.visit_select_clause(&select_stmt.select_clause);
        self.visit_from_clause(&select_stmt.from_clause);

        self.visit_indexes(&select_stmt.indexes);
        self.visit_where_clause(&select_stmt.where_clause);
        self.visit_limit_and_offset(select_stmt.limit, select_stmt.offset);
        self.visit_locking(&select_stmt.locking);
        self
    }

    pub fn visit_locking(&mut self, locking: &Option<(Lock, Wait)>) -> &mut Self {
        if self.dialect.caps().select_for_update
            && let Some((lock, wait)) = locking
        {
            match lock {
                Lock::Update => self.push(" FOR UPDATE"),
                Lock::UpdateOf(n) => self.push(" FOR UPDATE OF ").visit_named_table(n),
                Lock::Share => self.push(" FOR SHARE"),
            };
            match wait {
                Wait::DEFAULT => self.noop(),
                Wait::NoWait => self.push(" NOWAIT"),
                Wait::SkipLocked => self.push(" SKIP LOCKED"),
            };
        }
        self
    }

    pub fn visit_where_clause(&mut self, where_clause: &Vec<Expr>) -> &mut Self {
        let mut iter = where_clause.iter();
        if let Some(f) = iter.next() {
            self.push(" WHERE ");
            self.visit_expr(f, false);
            for f in iter {
                self.push(" AND ");
                self.visit_expr(f, false);
            }
        }
        self
    }

    pub fn visit_select_clause(&mut self, select_clause: &Vec<SelectItem>) -> &mut Self {
        let mut iter = select_clause.iter();
        if let Some(item) = iter.next() {
            self.visit_select_item(item);
            for item in iter {
                self.push(", ");
                self.visit_select_item(item);
            }
        } else {
            self.push("*");
        }
        self
    }

    pub fn visit_from_clause(&mut self, from_clause: &Vec<TableRef>) -> &mut Self {
        let mut iter = from_clause.iter();
        if let Some(t) = iter.next() {
            self.push(" FROM ");
            self.visit_table_ref(t);
            for t in iter {
                self.push(", ");
                self.visit_table_ref(t);
            }
        }
        self
    }
    pub fn visit_limit_and_offset(&mut self, limit: Option<usize>, offset: Option<usize>) {
        if let Some(n) = limit {
            self.push(" LIMIT ").push(&n.to_string());
        }
        if self.dialect.caps().standalone_offset || limit.is_some() {
            if let Some(n) = offset {
                self.push(" OFFSET ").push(&n.to_string());
            }
        }
    }

    pub fn visit_table_ref(&mut self, table_ref: &TableRef) -> &mut Self {
        match table_ref {
            TableRef::Named { table, alias } => self.visit_named_table(table).visit_alias(alias),
            TableRef::Subquery { subquery, alias } => self.visit_subquery(subquery).visit_alias(&Some(alias.clone())),
            TableRef::Join { join, alias } => self.visit_join(join).visit_alias(alias),
        }
    }

    pub fn visit_named_table(&mut self, table: &NamedTable) -> &mut Self {
        self.push_quote(table.name())
    }

    pub fn visit_subquery(&mut self, subquery: &Subquery) -> &mut Self {
        self.push("(");
        self.visit_select_statement(subquery.select_statement());
        self.push(")")
    }

    pub fn visit_join(&mut self, join: &Join) -> &mut Self {
        self
    }

    pub fn visit_select_item(&mut self, item: &SelectItem) -> &mut Self {
        match item {
            SelectItem::Wildcard => self.push("*"),
            SelectItem::QualifiedWildcard(t) => self.push_quote(t).push("*"),
            SelectItem::Expr { expr, alias } => self.visit_expr(expr, true).visit_alias(alias),
        }
    }

    pub fn visit_expr(&mut self, expr: &Expr, inline: bool) -> &mut Self {
        match expr {
            Expr::Column(c) => self.visit_column_ref(c),
            Expr::Literal(l) => self.visit_literal(l, inline),
            Expr::Binary { left, op, right } => self.visit_expr(left, inline).visit_op(op).visit_expr(right, inline),
            Expr::In { expr, list, negated } => self
                .visit_expr(expr, inline)
                .visit_op(if *negated { &NOT_IN } else { &IN })
                .visit_expr_list(list, inline),
            Expr::Unary { op, expr } => self.visit_op(op).visit_expr(expr, inline),
            _ => panic!("不支持"),
        }
    }

    pub fn visit_expr_list(&mut self, expr_list: &Vec<Expr>, inline: bool) -> &mut Self {
        self.push("(");
        let mut iter = expr_list.iter();
        if let Some(expr) = iter.next() {
            self.visit_expr(expr, inline);
        }
        for expr in iter {
            self.push(", ").visit_expr(expr, inline);
        }
        self.push(")");
        self
    }

    #[inline]
    pub fn visit_op(&mut self, op: &Op) -> &mut Self {
        self.push(" ").push(op.as_ref()).push(" ")
    }

    pub fn visit_distinct(&mut self, distinct: &Distinct) -> &mut Self {
        match distinct {
            Distinct::None => self,
            Distinct::Simple => self.push("DISTINCT "),
            Distinct::On(cols) => {
                if self.dialect.caps().distinct_on {
                    self.push("DISTINCT ON (");
                    let mut iter = cols.iter();
                    if let Some(item) = iter.next() {
                        self.visit_column_ref(item);
                        for item in iter {
                            self.push(", ");
                            self.visit_column_ref(item);
                        }
                    }
                    self.push(") ")
                } else {
                    self.push("DISTINCT ")
                }
            },
        }
    }

    pub fn visit_column_ref(&mut self, col: &ColumnRef) -> &mut Self {
        if let Some(q) = &col.qualifier {
            self.push_quote(q).push(".");
        }
        self.push_quote(&col.name);
        self
    }

    pub fn visit_literal(&mut self, lit: &Literal, inline: bool) -> &mut Self {
        if !inline && !lit.is_null() {
            self.builder.bind(lit.clone(), &self.dialect);
            return self;
        }

        match lit {
            Literal::Null => self.push("NULL"),
            Literal::Int(v) => self.push(&v.to_string()),
            Literal::Float(v) => self.push(&v.to_string()),
            Literal::Bool(v) => self.push(self.dialect.bool_str(*v)),
            Literal::String(v) => self.push("'").push_escape(&v).push("'"),
            Literal::Date(v) => self.push("'").push(&v.to_string()).push("'"),
            Literal::DateTime(v) => self.push("'").push(&v.to_string()).push("'"),
            Literal::Time(v) => self.push("'").push(&v.to_string()).push("'"),
        }
    }

    fn visit_alias(&mut self, alias: &Option<Alias>) -> &mut Self {
        if let Some(a) = alias {
            self.push(" AS ");
            self.push_quote(a.name());
        }
        self
    }

    #[inline]
    pub fn finish(&self) -> (&str, &Vec<Literal>) {
        (&self.builder.buff, &self.builder.binder)
    }

    #[inline]
    pub fn reset(&mut self) -> &mut Self {
        self.builder.clear();
        self
    }
    #[inline]
    fn visit_indexes(&mut self, indexes: &[Index]) -> &mut Self {
        self.dialect.render_force_index_hint(indexes, &mut self.builder);
        self
    }

    #[inline]
    fn push(&mut self, v: &str) -> &mut Self {
        self.builder.push(v);
        self
    }

    #[inline]
    fn push_escape(&mut self, v: &str) -> &mut Self {
        let v = v.replace("'", "''").replace("\\", "\\\\");
        self.builder.push(&v);
        self
    }

    #[inline]
    fn push_quote(&mut self, v: &str) -> &mut Self {
        let char = self.dialect.quote_char();
        self.push(char).push(v).push(char)
    }

    #[inline]
    fn noop(&mut self) -> &mut Self {
        self
    }
}
