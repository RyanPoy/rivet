<<<<<<< HEAD:rivet-orm/src/sequel/term/literal.rs
use crate::sequel::term::calendar::{Date, DateTime, Time};
=======
use crate::ast2::term::calendar::{Date, DateTime, Time};
use crate::ast2::term::expr::Expr;
use crate::ast2::term::select_item::SelectItem;
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/literal.rs

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
<<<<<<< HEAD:rivet-orm/src/sequel/term/literal.rs
=======
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        Expr::Literal(self).alias(alias)
    }
>>>>>>> 8774772226ca2687befa563f5ff2fc9ff202e17c:rivet-orm/src/ast2/term/literal.rs
    pub fn is_null(&self) -> bool {
        match &self {
            Self::Null => true,
            _ => false,
        }
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
