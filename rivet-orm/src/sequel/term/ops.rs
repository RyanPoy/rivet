#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
    Like,
    NotLike,
    In,
    NotIn,
    Is,
    IsNot,
}
impl BinaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",

            Self::Eq => "=",
            Self::NotEq => "<>",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",

            Self::And => "AND",
            Self::Or => "OR",

            Self::Like => "LIKE",
            Self::NotLike => "NOT LIKE",
            Self::In => "IN",
            Self::NotIn => "NOT IN",
            Self::Is => "IS",
            Self::IsNot => "IS NOT",
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Self::Or => 10,
            Self::And => 20,
            _ => 30, // 比较运算和算术运算更高
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg, // -
    Pos, // +
}
impl UnaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Not => "NOT",
            Self::Neg => "-",
            Self::Pos => "+",
        }
    }
}
