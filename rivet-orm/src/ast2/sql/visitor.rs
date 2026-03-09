use crate::ast2::sql::builder::Builder;
use crate::ast2::sql::dialect::{Dialect, MySQL, PostgreSQL, SQLite};
use crate::ast2::statement::select::SelectStatement;
use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::distinct::Distinct;
use crate::ast2::term::expr::Expr;
use crate::ast2::term::func::{Func, FuncArg};
use crate::ast2::term::index::Index;
use crate::ast2::term::join::Join;
use crate::ast2::term::literal::Literal;
use crate::ast2::term::lock::{Lock, Wait};
use crate::ast2::term::ops::{IN, NOT_IN, Op};
use crate::ast2::term::select_item::SelectItem;
use crate::ast2::term::table::{Table, TableInner};
use std::collections::HashMap;
use std::sync::Arc;

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
    alias_mapping: HashMap<usize, (usize, Option<String>)>,
}

impl<D: Dialect> Visitor<D> {
    pub fn new(dialect: D) -> Self {
        Self {
            builder: Builder::new(),
            dialect,
            alias_mapping: HashMap::new(),
        }
    }

    fn register_tables(&mut self, stmt: &SelectStatement) {
        // 1. 处理 FROM 子句（这是产生新别名的主要地方）
        for table in &stmt.from_clause {
            self.register_table_inner(&table);
        }

        // 2. 处理 SELECT 子句中的子查询
        for item in &stmt.select_clause {
            if let SelectItem::Expr(expr, _) = item {
                self.register_table_from_expr(expr);
            }
        }

        // 3. 处理 WHERE 子句中的子查询 (测试用例中的 EXISTS 在这里)
        // if let Some(where_expr) = &stmt.where_clause {
        //     self.register_table_from_expr(where_expr);
        // }
    }
    fn register_table_from_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Subquery(sq) => self.register_tables(sq),
            Expr::Func(func) => {
                for arg in &func.args {
                    match arg {
                        FuncArg::Expr { expr, .. } => self.register_table_from_expr(expr),
                        FuncArg::Subquery(sq) => self.register_tables(sq),
                        FuncArg::Wildcard => {},
                    }
                }
            },
            Expr::Binary { left, right, .. } => {
                self.register_table_from_expr(left);
                self.register_table_from_expr(right);
            },
            Expr::Unary { expr, .. } => self.register_table_from_expr(expr),
            Expr::In { expr, list, .. } => {
                self.register_table_from_expr(expr);
                for e in list {
                    self.register_table_from_expr(e);
                }
            },
            _ => {},
        }
    }
    fn register_table_inner(&mut self, table: &Table) {
        let inner = &table.inner;
        let alias = &table.alias;

        match &inner.as_ref() {
            TableInner::Named(name) => {
                let addr = Arc::as_ptr(inner) as usize;
                if !self.alias_mapping.contains_key(&addr) {
                    let n = self.alias_mapping.len() + 1;
                    self.alias_mapping.insert(addr, (n, alias.clone()));
                }
            },
            TableInner::Subquery(sq) => {
                self.register_tables(sq);
            },
            TableInner::Join(join) => {
                // 递归处理 Join 的左右两边
                self.register_table_inner(&join.left);
                self.register_table_inner(&join.right);
            },
        };
    }

    fn alias_of(&self, table_inner: &Arc<TableInner>) -> Option<String> {
        let addr = Arc::as_ptr(table_inner) as usize;
        if let Some((num, alias)) = self.alias_mapping.get(&addr) {
            if let Some(a) = alias {
                alias.clone()
            } else {
                Some(format!("t{}", num))
            }
        } else {
            None
        }
    }

    pub fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> &mut Self {
        self.register_tables(&select_stmt);

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
                Lock::UpdateOf(n) => self.push(" FOR UPDATE OF ").push_quote(n),
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

    pub fn visit_from_clause(&mut self, from_clause: &Vec<Table>) -> &mut Self {
        let mut iter = from_clause.iter();
        if let Some(t) = iter.next() {
            self.push(" FROM ");
            self.visit_table(t);
            for t in iter {
                self.push(", ");
                self.visit_table(t);
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

    pub fn visit_table(&mut self, table: &Table) -> &mut Self {
        match &table.inner.as_ref() {
            TableInner::Named(name) => self.push_quote(name),
            TableInner::Subquery(subquery) => self.push("(").visit_select_statement(subquery).push(")"),
            TableInner::Join(join) => self.visit_join(join),
        };
        let alias = self.alias_of(&table.inner);
        self.visit_alias(&alias)
    }

    pub fn visit_join(&mut self, join: &Join) -> &mut Self {
        self
    }

    pub fn visit_select_item(&mut self, item: &SelectItem) -> &mut Self {
        match item {
            SelectItem::All(None) => self.push("*"),
            SelectItem::All(Some(table)) => {
                let alias = self.alias_of(table);
                if let Some(alias) = alias {
                    self.push_quote(&alias).push(".");
                }
                self.push("*")
            },
            SelectItem::Expr(expr, alias) => self.visit_expr(expr, true).visit_alias(alias),
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
            Expr::Func(f) => self.visit_func(f, inline),
            Expr::Subquery(sq) => self.visit_select_statement(sq),
        }
    }

    pub fn visit_func(&mut self, f: &Func, inline: bool) -> &mut Self {
        self.push(&f.name).push("(");
        let mut iter = f.args.iter();
        if let Some(arg) = iter.next() {
            self.visit_func_arg(arg, inline);
            for arg in iter {
                self.push(", ");
                self.visit_func_arg(arg, inline);
            }
        }
        self.push(")")
    }

    pub fn visit_func_arg(&mut self, arg: &FuncArg, inline: bool) -> &mut Self {
        match arg {
            FuncArg::Expr { expr, distinct } => {
                if *distinct {
                    self.push("DISTINCT ");
                }
                self.visit_expr(expr, inline)
            },
            FuncArg::Wildcard => self.push("*"),
            FuncArg::Subquery(sq) => self.visit_select_statement(sq),
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
        if let Some(table) = &col.table_inner {
            let alias = self.alias_of(table);
            if let Some(alias) = alias {
                self.push_quote(&alias).push(".");
            }
        }
        self.push_quote(&col.name)
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

    fn visit_alias(&mut self, alias: &Option<String>) -> &mut Self {
        if let Some(a) = alias {
            self.push(" AS ");
            self.push_quote(a);
        }
        self
    }

    #[inline]
    pub fn finish(&self) -> (String, Vec<Literal>) {
        (self.builder.buff.clone(), self.builder.binder.clone())
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
