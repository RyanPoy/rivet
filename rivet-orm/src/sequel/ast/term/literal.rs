use crate::sequel::ast::Scalar;
use crate::sequel::build::Binder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Single(Scalar),
    List(Vec<Scalar>),
    Raw(String),
}

impl Literal {
    pub fn build(&self, binder: &mut Binder) -> String {
        match self {
            Self::Single(s) => binder.format_literal(s.clone()),
            Self::List(vs) => {
                let placeholders: Vec<String> = vs.iter().map(|s| binder.format_literal(s.clone())).collect();
                format!("({})", placeholders.join(","))
            }
            Self::Raw(s) => s.to_string(),
        }
    }
}
