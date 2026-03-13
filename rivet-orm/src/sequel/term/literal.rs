use crate::sequel::term::calendar::{Date, DateTime, Time};
use crate::sequel::term::expr::Expr;
use crate::sequel::term::select_item::SelectItem;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Date(Date),
    DateTime(DateTime),
    Time(Time),
}

impl Literal {
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        Expr::Literal(self).alias(alias)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

// 空
impl From<()> for Literal {
    fn from(_: ()) -> Self {
        Literal::Null
    }
}

// 整数
impl From<i8> for Literal {
    fn from(v: i8) -> Self {
        Literal::Int(v as i64)
    }
}
impl From<i16> for Literal {
    fn from(v: i16) -> Self {
        Literal::Int(v as i64)
    }
}
impl From<i32> for Literal {
    fn from(v: i32) -> Self {
        Literal::Int(v as i64)
    }
}
impl From<i64> for Literal {
    fn from(v: i64) -> Self {
        Literal::Int(v)
    }
}

// 浮点
impl From<f32> for Literal {
    fn from(v: f32) -> Self {
        Literal::Float(v as f64)
    }
}
impl From<f64> for Literal {
    fn from(v: f64) -> Self {
        Literal::Float(v)
    }
}

// 字符串
impl From<&str> for Literal {
    fn from(v: &str) -> Self {
        Literal::String(v.into())
    }
}
impl From<String> for Literal {
    fn from(v: String) -> Self {
        Literal::String(v)
    }
}

// 布尔值
impl From<bool> for Literal {
    fn from(v: bool) -> Self {
        Literal::Bool(v)
    }
}

// 时间和日期
impl From<Date> for Literal {
    fn from(v: Date) -> Self {
        Literal::Date(v)
    }
}
impl From<DateTime> for Literal {
    fn from(v: DateTime) -> Self {
        Literal::DateTime(v)
    }
}
impl From<Time> for Literal {
    fn from(v: Time) -> Self {
        Literal::Time(v)
    }
}
