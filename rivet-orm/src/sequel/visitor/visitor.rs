use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::column::Column;
use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{Func, FuncArg};
use crate::sequel::term::index::{Index, Indexes};
use crate::sequel::term::join::{Join, JoinType};
use crate::sequel::term::lock::{Lock, Locking, Wait};
use crate::sequel::term::ops::{BinaryOp, UnaryOp};
use crate::sequel::term::param::{Param, ParamData};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::{Table, TableInner};
use crate::sequel::visitor::alias_cache::AliasCache;
use crate::sequel::visitor::builder::Builder;
use crate::sequel::visitor::dialect::Dialect;
use crate::sequel::visitor::dialect::caps::{CountDistinctCap, IndexFormat};
use crate::sequel::visitor::dialect::mysql::MySQL;
use crate::sequel::visitor::dialect::postgre::PostgreSQL;
use crate::sequel::visitor::dialect::sqlite::SQLite;
use crate::sequel::visitor::rewriter::rewrite_count_distinct;

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
        for where_expr in &stmt.where_clause {
            self.register_table_from_expr(where_expr);
        }
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
                self.alias_cache.add(inner, "sq".to_string(), None);
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
        // let select_stmt = normalize(select_stmt);
        let select_stmt = rewrite_count_distinct(select_stmt, &self.dialect);

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

    pub fn visit_locking(&mut self, locking: &Option<Locking>) -> &mut Self {
        if self.dialect.caps().select_with_locking {
            if let Some(locking) = locking {
                if let Some(lock) = &locking.lock {
                    match lock {
                        Lock::Share => self.push(" FOR SHARE"),
                        Lock::Update => self.push(" FOR UPDATE"),
                        Lock::UpdateOf(tables) => {
                            let mut iter = tables.iter();
                            if let Some(table) = iter.next() {
                                self.push(" FOR UPDATE OF ");
                                self.push_quote(&table.visible_name());
                            }
                            for table in iter {
                                self.push(", ").push_quote(&table.visible_name());
                            }
                            self.noop()
                        },
                    };
                    if let Some(wait) = &locking.wait {
                        match wait {
                            Wait::Default => self.noop(),
                            Wait::NoWait => self.push(" NOWAIT"),
                            Wait::SkipLocked => self.push(" SKIP LOCKED"),
                        };
                    }
                } else if let Some(wait) = &locking.wait {
                    match wait {
                        Wait::Default => self.push(" FOR UPDATE"),
                        Wait::NoWait => self.push(" FOR UPDATE NOWAIT"),
                        Wait::SkipLocked => self.push(" FOR UPDATE SKIP LOCKED"),
                    };
                }
            }
        }
        self
    }
    pub fn visit_where_clause(&mut self, where_clause: &Vec<Expr>) -> &mut Self {
        let mut iter = where_clause.iter();
        if let Some(f) = iter.next() {
            self.push(" WHERE ");
            match f {
                Expr::Param(lit) => self.visit_param(lit).push(" = ").visit_param(lit),
                _ => self.visit_expr(f, 0),
            };
            for f in iter {
                self.push(" AND ");
                match f {
                    Expr::Param(lit) => self.visit_param(lit).push(" = ").visit_param(lit),
                    _ => self.visit_expr(f, 0),
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
        let add_paren = match &join.right.inner.as_ref() {
            TableInner::Join(join) => true,
            _ => false,
        };
        if add_paren {
            self.push("(");
        }
        self.visit_table(&join.right);
        if add_paren {
            self.push(")");
        }
        if let Some(on) = &join.on {
            self.push(" ON ").visit_expr(on, 0);
        }
        self
    }

    pub fn visit_select_item(&mut self, item: &SelectItem) -> &mut Self {
        match item {
            SelectItem { expr, alias } => self.visit_expr(expr, 0).visit_alias(alias),
        }
    }

    pub fn visit_expr(&mut self, expr: &Expr, parent_precedence: i32) -> &mut Self {
        let current_precedence = expr.precedence();
        let need_parens = current_precedence < parent_precedence;
        if need_parens {
            self.push("(");
        }
        match expr {
            Expr::Column(c) => self.visit_column_ref(c),
            Expr::Param(l) => self.visit_param(l),
            Expr::Binary { left, op, right } => match &**left {
                Expr::Param(l) => self
                    .visit_param(&l)
                    .visit_binary_op(op)
                    .visit_expr(right, current_precedence),
                _ => self
                    .visit_expr(left, current_precedence)
                    .visit_binary_op(op)
                    .visit_expr(right, current_precedence),
            },
            Expr::In { expr, list, negated } => self
                .visit_expr(expr, current_precedence)
                .visit_binary_op(if *negated { &BinaryOp::NotIn } else { &BinaryOp::In })
                .push("(")
                .visit_expr_list(list, 0)
                .push(")"),
            Expr::Unary { op, expr } => self.visit_unary_op(op).visit_expr(expr, current_precedence),
            Expr::Func(f) => self.visit_func(f),
            Expr::Subquery(sq) => self.push("(").visit_select_statement(sq).push(")"),
        };
        if need_parens {
            self.push(")");
        }
        self
    }
    pub fn visit_func(&mut self, f: &Func) -> &mut Self {
        if !f.distinct {
            // isn't distinct
            return self.push(&f.name).push("(").push_func_args(&f.args).push(")");
        }

        if f.args.len() <= 1 || !f.name.eq_ignore_ascii_case("count") {
            // distinct, but not count and multiple columns。
            return self.push(&f.name).push("(DISTINCT ").push_func_args(&f.args).push(")");
        }
        // count distinct multiple columns
        self.push(&f.name).push("(DISTINCT ");
        match self.dialect.caps().count_distinct {
            CountDistinctCap::Merge => {
                self.push("(").push_func_args(&f.args).push(")");
            },
            CountDistinctCap::Extend => {
                self.push_func_args(&f.args);
            },
            _ => unreachable!(),
        }
        self.push(")")
    }

    fn push_func_args(&mut self, args: &[FuncArg]) -> &mut Self {
        let mut iter = args.iter();
        if let Some(arg) = iter.next() {
            self.visit_func_arg(arg);
        }
        for arg in iter {
            self.push(", ");
            self.visit_func_arg(arg);
        }
        self
    }

    pub fn visit_func_arg(&mut self, arg: &FuncArg) -> &mut Self {
        match arg {
            FuncArg::Expr(expr) => self.visit_expr(expr, 0),
            FuncArg::Wildcard => self.push("*"),
        }
    }

    pub fn visit_expr_list(&mut self, expr_list: &Vec<Expr>, parent_precedence: i32) -> &mut Self {
        let mut iter = expr_list.iter();
        if let Some(expr) = iter.next() {
            self.visit_expr(expr, 0);
        }
        for expr in iter {
            self.push(", ").visit_expr(expr, 0);
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
            Distinct::None => self.noop(),
            Distinct::All => self.push("DISTINCT "),
            Distinct::On(_) if !self.dialect.caps().distinct_on => self.push("DISTINCT "),
            Distinct::On(cols) => self.push("DISTINCT ON (").visit_expr_list(cols, 0).push(") "),
        }
    }

    pub fn visit_column_ref(&mut self, col: &Column) -> &mut Self {
        let alias = self.alias_cache.alias_of(&col.table_inner);
        if let Some(alias) = alias {
            self.push_quote(&alias).push(".");
        }
        self.push_quote(&col.name)
    }

    pub fn visit_param(&mut self, p: &Param) -> &mut Self {
        match p {
            Param::Null => self.push("NULL"),
            Param::Inline(data) => self.bind(data.clone()),
            Param::Data(data) => match data {
                ParamData::Int(v) => self.push(&v.to_string()),
                ParamData::Float(v) => self.push(&v.to_string()),
                ParamData::Bool(v) => self.push(self.dialect.bool_str(*v)),
                ParamData::String(v) => self.push("'").push_escape(&v).push("'"),
                ParamData::Date(v) => self.push("'").push(&v.to_string()).push("'"),
                ParamData::DateTime(v) => self.push("'").push(&v.to_string()).push("'"),
                ParamData::Time(v) => self.push("'").push(&v.to_string()).push("'"),
            },
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
    pub fn finish(&self) -> (String, Vec<ParamData>) {
        (self.builder.buff.clone(), self.builder.binder.clone())
    }

    #[inline]
    fn visit_indexes(&mut self, indexes: &Indexes) -> &mut Self {
        let caps = self.dialect.caps();
        self._visit_index_by(caps.index_cap.force, &indexes.force);
        self._visit_index_by(caps.index_cap.use_, &indexes.use_);
        self._visit_index_by(caps.index_cap.ignore, &indexes.ignore);
        self
    }

    fn _visit_index_by(&mut self, render: Option<IndexFormat>, indexes: &Vec<Index>) -> &mut Self {
        if let Some(render) = render {
            let mut iter = indexes.iter();
            if let Some(index) = iter.next() {
                self.push(" ").push(render.before).push(" ");
                self.push_quote(&index.to_string());
                if render.support_multiple {
                    for index in iter {
                        self.push(", ").push_quote(&index.to_string());
                    }
                }
                self.push(render.after);
            }
        }
        self
    }
    #[inline]
    fn push(&mut self, v: &str) -> &mut Self {
        self.builder.push(v);
        self
    }

    #[inline]
    fn bind(&mut self, v: ParamData) -> &mut Self {
        self.builder.bind(v, &self.dialect);
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
