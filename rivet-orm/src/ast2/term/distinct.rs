use crate::ast2::term::column_ref::ColumnRef;

#[derive(Clone, Debug)]
pub enum Distinct {
    None,
    Simple,
    On(Vec<ColumnRef>),
}