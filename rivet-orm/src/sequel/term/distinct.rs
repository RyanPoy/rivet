use crate::sequel::term::column::Column;

#[derive(Clone, Debug)]
pub enum Distinct {
    None,
    Simple,
    On(Vec<Column>),
}