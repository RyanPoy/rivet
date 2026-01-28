use crate::ast::Expr;
use crate::ast::Source;
use crate::ast::{Direction, Order};
use crate::ast::{Operand, Value};

#[derive(Clone)]
pub struct SelectStatement {
    pub distinct: bool,
    pub select: Vec<Operand>,
    pub from: Option<Source>,
    pub _where: Option<Expr>,
    pub group: Vec<Operand>,
    pub having: Option<Expr>,
    pub order: Vec<Order>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
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
    //
    // pub fn to_sql(&self) -> (String, Vec<Value>) {
    //     let mut params = vec![];
    //     let mut parts = vec![];
    //
    //     // 1. SELECT
    //     parts.push("SELECT ".to_string());
    //     if self.select.is_empty() {
    //         parts.push("* ".to_string());
    //     } else {
    //         for operand in self.select {
    //             parts.push
    //         }
    //     }
    //     for operand in self.select {
    //         parts.push
    //     }
    //     let cols: Vec<String> = self.select.iter().map(|c| c.render_sql(&mut ctx)).collect();
    //     select_sql.push_str(&cols.join(", "));
    //     parts.push(select_sql);
    //
    //     // 2. FROM
    //     if let Some(ref src) = self.from {
    //         parts.push(format!("FROM {}", src.render_sql(&mut ctx)));
    //     }
    //
    //     // 3. WHERE
    //     if let Some(ref expr) = self._where {
    //         parts.push(format!("WHERE {}", expr.render_sql(&mut ctx)));
    //     }
    //
    //     // 4. LIMIT (Limit 的值也应该作为参数绑定)
    //     if let Some(limit) = self.limit {
    //         parts.push(format!("LIMIT {}", ctx.push_param(Value::USize(limit))));
    //     }
    //
    //     (parts.join(" "), ctx.params)
    // }
}
