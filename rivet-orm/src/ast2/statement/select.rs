use crate::ast2::term::column_ref::ColumnRef;
use crate::ast2::term::distinct::Distinct;
use crate::ast2::term::select_item::SelectItem;
use crate::ast2::term::table_ref::TableRef;

/// SelectStatement
/// ├─ select_clause: Vec<SelectItem>
/// │    ├─ SelectItem::Expr { expr: Expr, alias: Option<String> }
/// │    │      ├─ Expr::Column(ColumnRef)
/// │    │      ├─ Expr::Literal(Literal)
/// │    │      ├─ Expr::Unary { op, expr: Box<Expr> }
/// │    │      ├─ Expr::Binary { left: Box<Expr>, right: Box<Expr>, op }
/// │    │      ├─ Expr::Func { name, args: Vec<FuncArg> }
/// │    │      └─ Expr::Subquery(Box<SelectStatement>)  ← 子查询表达式，返回单值
/// │    └─ SelectItem::Wildcard / QualifiedWildcard
/// │
/// └─ from_clause: Vec<TableRef>
///      ├─ TableRef::NamedTable(NamedTable)
///      ├─ TableRef::DerivedTable(DerivedTable)
///      │      ├─ stmt: Box<SelectStatement>     ← 子查询返回表
///      │      └─ alias: Option<String>
///      └─ TableRef::JoinedTable(JoinedTable)
///             ├─ left: Box<TableRef>
///             ├─ right: Box<TableRef>
///             ├─ join_type: JoinType
///             └─ condition: Option<Expr>          ← ON 条件
#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub distinct: Distinct,
    pub select_clause: Vec<SelectItem>,
    pub from_clause: Vec<TableRef>,
}

impl SelectStatement {
    pub fn new() -> Self {
        Self { distinct: Distinct::None, select_clause: Vec::new(), from_clause: Vec::new() }
    }
    pub fn distinct(mut self) -> Self {
        self.distinct = Distinct::Simple;
        self
    }
    pub fn distinct_on(mut self, cols: Vec<ColumnRef>) -> Self {
        self.distinct = Distinct::On(cols);
        self
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
        self.from_clause.extend(ts.into_iter().map(|t| t.into()));
        self
    }

    pub fn select<C>(mut self, c: C) -> Self
    where
        C: Into<SelectItem>,
    {
        self.select_clause.push(c.into());
        self
    }

    pub fn select_many<C, I>(mut self, cs: I) -> Self
    where
        C: Into<SelectItem>,
        I: IntoIterator<Item = C>,
    {
        self.select_clause.extend(cs.into_iter().map(|c| c.into()));
        self
    }
}

#[cfg(test)]
#[path = "./select_test.rs"]
mod tests;
