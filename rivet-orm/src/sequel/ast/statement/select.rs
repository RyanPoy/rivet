use crate::sequel::ast::Expr;
use crate::sequel::ast::Source;
use crate::sequel::ast::{Direction, Order};
use crate::sequel::ast::{Operand, Value};
use crate::sequel::build::Binder;

#[derive(Clone)]
pub struct SelectStatement {
    pub distinct: bool,
    pub select: Vec<Operand>,
    pub from: Option<Source>,
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
            from: None,
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
        self.from = Some(source);
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
        let mut parts = Vec::new();

        // 1. SELECT 子句
        let mut select_clause = String::from("SELECT ");
        if self.distinct {
            select_clause.push_str("DISTINCT ");
        }
        if self.select.is_empty() {
            select_clause.push_str("*");
        } else {
            let cols: Vec<String> = self.select.iter().map(|col| col.build(binder)).collect();
            select_clause.push_str(&cols.join(", "));
        }
        parts.push(select_clause);

        // 2. FROM 子句
        if let Some(source) = &self.from {
            parts.push(format!("FROM {}", source.build(binder)));
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
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
