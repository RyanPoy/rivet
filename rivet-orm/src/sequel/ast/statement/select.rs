use crate::sequel::ast::Source;
use crate::sequel::ast::{Direction, Order};
use crate::sequel::ast::{Expr, Table};
use crate::sequel::ast::{Operand, Value};
use crate::sequel::build::Binder;

#[derive(Clone)]
pub struct SelectStatement {
    pub distinct: bool,
    pub select: Vec<Operand>,
    pub from: Vec<Source>,
    pub _where: Option<Expr>,
    pub group: Vec<Operand>,
    pub having: Option<Expr>,
    pub order: Vec<Order>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl SelectStatement {
    pub fn new() -> Self {
        SelectStatement {
            distinct: false,
            select: vec![],
            from: vec![],
            _where: None,
            group: vec![],
            having: None,
            order: vec![],
            limit: None,
            offset: None,
        }
    }

    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    pub fn select(mut self, col: Operand) -> Self {
        self.select.push(col);
        self
    }
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn offset(mut self, n: u32) -> Self {
        self.offset = Some(n);
        self
    }

    pub fn from(mut self, source: Source) -> Self {
        self.from.push(source);
        self
    }

    pub fn where_(mut self, expr: Expr) -> Self {
        self._where = Some(expr);
        self
    }
    pub fn order_by(mut self, column: Operand) -> Self {
        self.order.push(Order { column, direction: Direction::Asc });
        self
    }
    pub fn order_by_desc(mut self, column: Operand) -> Self {
        self.order.push(Order { column, direction: Direction::Desc });
        self
    }

    pub fn having(mut self, expr: Expr) -> Self {
        self.having = Some(expr);
        self
    }
    pub fn group_by(mut self, column: Operand) -> Self {
        self.group.push(column);
        self
    }

    pub fn to_sql(&self, binder: &mut Binder) -> (String, Vec<Value>) {
        let sql = self.build(binder);
        (sql, binder.params())
    }
    pub fn build(&self, binder: &mut Binder) -> String {
        // 0. 扫描 Alias
        let aliases = self.collect_aliases();

        let mut parts = Vec::new();

        // 1. SELECT 子句
        let mut select_clause = String::from("SELECT ");
        if self.distinct {
            select_clause.push_str("DISTINCT ");
        }
        if self.select.is_empty() {
            if aliases.len() == 1 {
                let s = format!("{}.*", aliases[0].1);
                select_clause.push_str(&s);
            } else {
                select_clause.push_str("*");
            }
        } else {
            let cols: Vec<String> = self
                .select
                .iter()
                .map(|operand| match operand {
                    Operand::Column(col) => {
                        let effective_table = match col.table {
                            Some(t) => Some(aliases.iter().find(|(o, _)| *o == t).map(|(_, a)| *a).unwrap_or(t)),
                            None => {
                                if aliases.len() == 1 {
                                    Some(aliases[0].1)
                                } else {
                                    None
                                }
                            }
                        };
                        let full_name = binder.quote_full(effective_table, col.name);
                        binder.with_alias(full_name, col.alias.as_deref())
                    }
                    Operand::Value(v) => binder.bind(v.clone()),
                })
                .collect();
            select_clause.push_str(&cols.join(", "));
        }
        parts.push(select_clause);

        // 2. FROM 子句
        if !self.from.is_empty() {
            let froms: Vec<String> = self.from.iter().map(|f| f.build(binder)).collect();
            parts.push(format!("FROM {}", froms.join(", ")));
        }

        // 3. WHERE 子句
        if let Some(expr) = &self._where {
            parts.push(format!("WHERE {}", expr.build(binder)));
        }

        // 4. GROUP BY 子句
        if !self.group.is_empty() {
            let groups: Vec<String> = self.group.iter().map(|g| g.build(binder)).collect();
            parts.push(format!("GROUP BY {}", groups.join(", ")));
        }

        // 5. HAVING 子句
        if let Some(expr) = &self.having {
            parts.push(format!("HAVING {}", expr.build(binder)));
        }

        // 6. ORDER BY 子句
        if !self.order.is_empty() {
            let orders: Vec<String> = self
                .order
                .iter()
                .map(|o| o.build(binder)) // 假设 Order 实现了该方法
                .collect();
            parts.push(format!("ORDER BY {}", orders.join(", ")));
        }

        // 7. LIMIT 子句 (将数字也绑定为参数)
        if let Some(limit) = self.limit {
            let placeholder = binder.bind(Value::U32(limit));
            parts.push(format!("LIMIT {}", placeholder));
        }

        // 8. OFFSET 子句
        if let Some(offset) = self.offset {
            let placeholder = binder.bind(Value::U32(offset));
            parts.push(format!("OFFSET {}", placeholder));
        }

        // 合并所有部分并完成构建
        let final_sql = parts.join(" ");
        final_sql
        // binder.finish(final_sql)
    }

    fn collect_aliases(&self) -> Vec<(&str, &str)> {
        let mut aliases: Vec<(&str, &str)> = Vec::new();
        for source in &self.from {
            match source {
                Source::Table(Table { schema, name, alias }) => {
                    if let Some(a) = alias {
                        aliases.push((name, a));
                    }
                }
                Source::SubQuery { query, alias } => {
                    if let Some(a) = alias {
                        aliases.push((a, a));
                    }
                }
                Source::Join { left, right, .. } => {
                    self.collect_from_source(&*left, &mut aliases);
                    self.collect_from_source(&*right, &mut aliases);
                }
            }
        }
        aliases
    }

    fn collect_from_source(&self, source: &Source, aliases: &mut Vec<(&str, &str)>) {
        match source {
            Source::Table(Table { schema, name, alias }) => {
                if let Some(a) = alias {
                    aliases.push((*name, *a))
                }
            }
            Source::Join { left, right, .. } => {
                self.collect_from_source(left, aliases);
                self.collect_from_source(right, aliases);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
