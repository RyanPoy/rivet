use crate::sequel::statement::select::SelectStatement;
use crate::sequel::term::distinct::Distinct;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::func::{Func, FuncArg};
use crate::sequel::term::select_item::SelectItem;
use crate::sequel::term::table::Table;
use crate::sequel::visitor::dialect::{CountDistinctCap, Dialect};

pub fn normalize(stmt: &SelectStatement) -> SelectStatement {
    let mut stmt = stmt.clone();
    for item in &mut stmt.select_clause {
        if let Expr::Func(ref mut f) = item.expr {
            if f.name.eq_ignore_ascii_case("count") && f.args.is_empty() {
                f.args.push(FuncArg::Wildcard);
            }
        }
    }
    stmt
}

fn find_count_distinct_multi(stmt: &SelectStatement) -> Option<(usize, &Func)> {
    stmt.select_clause.iter().enumerate().find_map(|(i, item)| {
        if let Expr::Func(ref f) = item.expr {
            if f.distinct && f.name.eq_ignore_ascii_case("count") && f.args.len() > 1 {
                return Some((i, f));
            }
        }
        None
    })
}

fn find_multi_count(stmt: &SelectStatement) -> Option<Vec<Expr>> {
    stmt.select_clause.iter().find_map(|item| {
        if let Expr::Func(f) = &item.expr {
            if f.name.eq_ignore_ascii_case("count") && f.distinct && f.args.len() > 1 {
                return Some(
                    f.args
                        .iter()
                        .filter_map(|a| match a {
                            FuncArg::Expr(e) => Some(e.clone()),
                            _ => None,
                        })
                        .collect(),
                );
            }
        }
        None
    })
}
fn collect_outer_columns(stmt: &SelectStatement) -> Vec<Expr> {
    stmt.select_clause
        .iter()
        .filter_map(|item| match &item.expr {
            Expr::Column(_) => Some(item.expr.clone()),
            _ => None,
        })
        .collect()
}
pub fn rewrite_count_distinct(stmt: SelectStatement, dialect: &dyn Dialect) -> SelectStatement {
    if dialect.caps().count_distinct == CountDistinctCap::Extend {
        return stmt;
    }

    let Some(count_cols) = find_multi_count(&stmt) else {
        return stmt;
    };

    let outer_cols = collect_outer_columns(&stmt);
    let mut sub_cols = count_cols.clone();
    sub_cols.append(&mut outer_cols.clone());

    let mut subquery = stmt.clone();
    subquery.select_clause = sub_cols.into_iter().map(|e| SelectItem::from(e)).collect();
    subquery.distinct = Distinct::All;
    subquery.from_clause = stmt.from_clause.clone();

    let sub_table = Table::from(subquery);

    let mut outer = stmt.clone();

    for item in &mut outer.select_clause {
        match &mut item.expr {
            Expr::Func(f) if f.name.eq_ignore_ascii_case("count") => {
                f.distinct = false;
                f.args = vec![FuncArg::Wildcard];
            },
            Expr::Column(c) => {
                c.table_inner = sub_table.inner.clone();
            },
            _ => {},
        }
    }

    outer.from_clause = sub_table;
    outer.distinct = Distinct::None;

    outer
}
