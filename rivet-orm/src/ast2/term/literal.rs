use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

