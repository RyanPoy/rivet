use crate::sequel::term::calendar::{Date, DateTime, Time};
use crate::sequel::term::comparable::Comparable;
use crate::sequel::term::expr::Expr;
use crate::sequel::term::select_item::SelectItem;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralData {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Date(Date),
    DateTime(DateTime),
    Time(Time),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Lit(LiteralData),
    Param(LiteralData),
    Null,
}

impl Literal {
    pub fn lit(v: impl Into<LiteralData>) -> Self {
        Self::Lit(v.into())
    }
    pub fn param(v: impl Into<LiteralData>) -> Self {
        Self::Param(v.into())
    }

    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        Expr::Literal(self).alias(alias)
    }

    #[inline]
    pub fn to_param(self) -> Self {
        match self {
            Literal::Lit(d) => Self::Param(d),
            _ => self,
        }
    }
    #[inline]
    pub fn to_lit(self) -> Self {
        match self {
            Literal::Param(d) => Self::Lit(d),
            _ => self,
        }
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn data(&self) -> Option<&LiteralData> {
        match self {
            Self::Lit(data) | Self::Param(data) => Some(data),
            Self::Null => None,
        }
    }
}

impl Comparable for Literal {
    fn into_expr(&self) -> Expr {
        Expr::Literal(self.clone())
    }
}

// 空
impl From<()> for Literal {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

// 整数
impl From<i8> for Literal {
    fn from(v: i8) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<i16> for Literal {
    fn from(v: i16) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<i32> for Literal {
    fn from(v: i32) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<i64> for Literal {
    fn from(v: i64) -> Self {
        Self::Param(LiteralData::Int(v))
    }
}

impl From<u8> for Literal {
    fn from(v: u8) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<u16> for Literal {
    fn from(v: u16) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<u32> for Literal {
    fn from(v: u32) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}
impl From<u64> for Literal {
    fn from(v: u64) -> Self {
        Self::Param(LiteralData::Int(v as i64))
    }
}

// 浮点
impl From<f32> for Literal {
    fn from(v: f32) -> Self {
        Self::Param(LiteralData::Float(v as f64))
    }
}
impl From<f64> for Literal {
    fn from(v: f64) -> Self {
        Self::Param(LiteralData::Float(v))
    }
}

// 字符串
impl From<&str> for Literal {
    fn from(v: &str) -> Self {
        Self::Param(LiteralData::String(v.into()))
    }
}
impl From<String> for Literal {
    fn from(v: String) -> Self {
        Self::Param(LiteralData::String(v))
    }
}

// 布尔值
impl From<bool> for Literal {
    fn from(v: bool) -> Self {
        Self::Param(LiteralData::Bool(v))
    }
}

// 时间和日期
impl From<Date> for Literal {
    fn from(v: Date) -> Self {
        Self::Param(LiteralData::Date(v))
    }
}
impl From<DateTime> for Literal {
    fn from(v: DateTime) -> Self {
        Self::Param(LiteralData::DateTime(v))
    }
}
impl From<Time> for Literal {
    fn from(v: Time) -> Self {
        Self::Param(LiteralData::Time(v))
    }
}

pub fn lit(v: impl Into<Literal>) -> Literal {
    v.into().to_lit()
}
