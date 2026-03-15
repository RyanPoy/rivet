use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{Func, FuncArg};
use crate::sequel::term::index::Index;
use crate::sequel::term::join::{Join, JoinType};
use crate::sequel::term::literal::Literal;
use crate::sequel::term::lock::{Lock, Wait};
use crate::sequel::term::ops::{BinaryOp, UnaryOp};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::{Table, TableInner};
use crate::sequel::visitor::alias_cache::AliasCache;
use crate::sequel::visitor::builder::Builder;
use crate::sequel::visitor::dialect::{CountDistinctCap, Dialect, MySQL, PostgreSQL, SQLite};
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
    alias_cache: AliasCache,
}

impl<D: Dialect> Visitor<D> {
    pub fn new(dialect: D) -> Self {
        Self {
            builder: Builder::new(),
            dialect,
            alias_cache: AliasCache::new(),
        }
    }

    fn register_tables(&mut self, stmt: &SelectStatement) {
        // 1. 处理 FROM 子句（这是产生新别名的主要地方）
        self.register_table_inner(&stmt.from_clause);

        // 2. 处理 SELECT 子句中的子查询
        for item in &stmt.select_clause {
            self.register_table_from_expr(&item.expr);
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
                        FuncArg::Expr(expr) => self.register_table_from_expr(expr),
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
        match &inner.as_ref() {
            TableInner::Named(name) => {
                self.alias_cache.add(inner, name.clone(), table.alias.clone());
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

    pub fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> &mut Self {
        self.register_tables(&select_stmt);

        self.push("SELECT ");
        self.visit_distinct(&select_stmt.distinct);
        self.visit_select_clause(&select_stmt.select_clause);

        self.push(" FROM ").visit_table(&select_stmt.from_clause);

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
            match f {
                Expr::Literal(lit) => self.visit_literal(lit, false).push(" = ").visit_literal(lit, false),
                _ => self.visit_expr(f, false, 0),
            };
            for f in iter {
                self.push(" AND ");
                match f {
                    Expr::Literal(lit) => self.visit_literal(lit, false).push(" = ").visit_literal(lit, false),
                    _ => self.visit_expr(f, false, 0),
                };
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
        let alias = self.alias_cache.alias_of(&table.inner);
        self.visit_alias(&alias)
    }

    pub fn visit_join(&mut self, join: &Join) -> &mut Self {
        self.visit_table(&join.left);
        match join.join_type {
            JoinType::Cross => self.push(" CROSS JOIN "),
            JoinType::Inner => self.push(" INNER JOIN "),
            JoinType::Left => self.push(" LEFT JOIN "),
            JoinType::Right => self.push(" RIGHT JOIN "),
            JoinType::Full => self.push(" FULL JOIN "),
        };
        self.visit_table(&join.right);
        if let Some(on) = &join.on {
            self.push(" ON ").visit_expr(on, false, 0);
        }
        self
    }

    pub fn visit_select_item(&mut self, item: &SelectItem) -> &mut Self {
        match item {
            SelectItem { expr, alias } => self.visit_expr(expr, true, 0).visit_alias(alias),
        }
    }

    pub fn visit_expr(&mut self, expr: &Expr, inline: bool, parent_precedence: i32) -> &mut Self {
        let current_precedence = expr.precedence();
        let need_parens = current_precedence < parent_precedence;
        if need_parens {
            self.push("(");
        }
        match expr {
            Expr::Column(c) => self.visit_column_ref(c),
            Expr::Literal(l) => self.visit_literal(l, inline),
            Expr::Binary { left, op, right } => self
                .visit_expr(left, inline, current_precedence)
                .visit_binary_op(op)
                .visit_expr(right, inline, current_precedence),
            Expr::In { expr, list, negated } => self
                .visit_expr(expr, inline, current_precedence)
                .visit_binary_op(if *negated { &BinaryOp::NotIn } else { &BinaryOp::In })
                .push("(")
                .visit_expr_list(list, inline, 0)
                .push(")"),
            Expr::Unary { op, expr } => self.visit_unary_op(op).visit_expr(expr, inline, current_precedence),
            Expr::Func(f) => self.visit_func(f, inline),
            Expr::Subquery(sq) => self.visit_select_statement(sq),
        };
        if need_parens {
            self.push(")");
        }
        self
    }
    pub fn visit_func(&mut self, f: &Func, inline: bool) -> &mut Self {
        if !f.distinct {
            // isn't distinct
            return self.push(&f.name).push("(").push_func_args(&f.args, inline).push(")");
        }

        if f.args.len() <= 1 || !f.name.eq_ignore_ascii_case("count") {
            // distinct, but not count and multiple columns。
            return self
                .push(&f.name)
                .push("(DISTINCT ")
                .push_func_args(&f.args, inline)
                .push(")");
        }
        // count distinct multiple columns
        self.push(&f.name).push("(DISTINCT ");
        match self.dialect.caps().count_distinct {
            CountDistinctCap::OneColumn => {
                if let Some(arg) = f.args.first() {
                    self.visit_func_arg(arg, inline);
                }
            },
            CountDistinctCap::Merge => {
                self.push("(").push_func_args(&f.args, inline).push(")");
            },
            CountDistinctCap::Extend => {
                self.push_func_args(&f.args, inline);
            },
        }
        self.push(")")
    }

    fn push_func_args(&mut self, args: &[FuncArg], inline: bool) -> &mut Self {
        let mut iter = args.iter();
        if let Some(arg) = iter.next() {
            self.visit_func_arg(arg, inline);
        }
        for arg in iter {
            self.push(", ");
            self.visit_func_arg(arg, inline);
        }
        self
    }

    pub fn visit_func_arg(&mut self, arg: &FuncArg, inline: bool) -> &mut Self {
        match arg {
            FuncArg::Expr(expr) => self.visit_expr(expr, inline, 0),
            FuncArg::Wildcard => self.push("*"),
        }
    }

    pub fn visit_expr_list(&mut self, expr_list: &Vec<Expr>, inline: bool, parent_precedence: i32) -> &mut Self {
        let mut iter = expr_list.iter();
        if let Some(expr) = iter.next() {
            self.visit_expr(expr, inline, 0);
        }
        for expr in iter {
            self.push(", ").visit_expr(expr, inline, 0);
        }
        self
    }

    #[inline]
    pub fn visit_binary_op(&mut self, op: &BinaryOp) -> &mut Self {
        self.push(" ").push(op.as_str()).push(" ")
    }
    #[inline]
    pub fn visit_unary_op(&mut self, op: &UnaryOp) -> &mut Self {
        self.push(op.as_str()).push(" ")
    }

    pub fn visit_distinct(&mut self, distinct: &Distinct) -> &mut Self {
        match distinct {
            Distinct::None => self,
            Distinct::All => self.push("DISTINCT "),
            Distinct::On(_) if !self.dialect.caps().distinct_on => self.push("DISTINCT "),
            Distinct::On(cols) => self.push("DISTINCT ON (").visit_expr_list(cols, true, 0).push(") "),
        }
    }

    pub fn visit_column_ref(&mut self, col: &Column) -> &mut Self {
        let alias = self.alias_cache.alias_of(&col.table_inner);
        if let Some(alias) = alias {
            self.push_quote(&alias).push(".");
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
