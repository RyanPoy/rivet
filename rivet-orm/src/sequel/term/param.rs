use crate::prelude::*;
use crate::sequel::term::calendar::{Date, DateTime, Time};
use crate::sequel::term::expr::Expr;
use crate::sequel::term::select_item::SelectItem;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ParamData {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Date(Date),
    DateTime(DateTime),
    Time(Time),
}

#[derive(Clone, Debug)]
pub enum Param {
    Literal(ParamData),
    Value(ParamData),
    Null,
}

impl Param {
    pub fn alias(self, alias: impl Into<String>) -> SelectItem {
        Expr::Param(self).alias(alias)
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn data(&self) -> Option<&ParamData> {
        match self {
            Self::Literal(data) | Self::Value(data) => Some(data),
            Self::Null => None,
        }
    }
}

impl Comparable for Param {
    fn into_expr(&self) -> Expr {
        Expr::Param(self.clone())
    }
}

// 空
impl From<()> for Param {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

// 整数
impl From<i8> for Param {
    fn from(v: i8) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<i16> for Param {
    fn from(v: i16) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<i32> for Param {
    fn from(v: i32) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<i64> for Param {
    fn from(v: i64) -> Self {
        Self::Value(ParamData::Int(v))
    }
}

impl From<u8> for Param {
    fn from(v: u8) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<u16> for Param {
    fn from(v: u16) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<u32> for Param {
    fn from(v: u32) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}
impl From<u64> for Param {
    fn from(v: u64) -> Self {
        Self::Value(ParamData::Int(v as i64))
    }
}

// 浮点
impl From<f32> for Param {
    fn from(v: f32) -> Self {
        Self::Value(ParamData::Float(v as f64))
    }
}
impl From<f64> for Param {
    fn from(v: f64) -> Self {
        Self::Value(ParamData::Float(v))
    }
}

// 字符串
impl From<&str> for Param {
    fn from(v: &str) -> Self {
        Self::Value(ParamData::String(v.into()))
    }
}
impl From<String> for Param {
    fn from(v: String) -> Self {
        Self::Value(ParamData::String(v))
    }
}

// 布尔值
impl From<bool> for Param {
    fn from(v: bool) -> Self {
        Self::Value(ParamData::Bool(v))
    }
}

// 时间和日期
impl From<Date> for Param {
    fn from(v: Date) -> Self {
        Self::Value(ParamData::Date(v))
    }
}
impl From<DateTime> for Param {
    fn from(v: DateTime) -> Self {
        Self::Value(ParamData::DateTime(v))
    }
}
impl From<Time> for Param {
    fn from(v: Time) -> Self {
        Self::Value(ParamData::Time(v))
    }
}

pub fn lit(v: impl Into<Param>) -> Param {
    match v.into() {
        Param::Value(d) => Param::Literal(d),
        p => p,
    }
}
