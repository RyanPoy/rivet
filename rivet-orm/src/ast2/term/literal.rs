#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Literal>),
}
