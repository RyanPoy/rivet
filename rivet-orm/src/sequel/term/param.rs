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

macro_rules! impl_from_int_for_param {
    ($($t:ty),*) => {
        $(impl From<$t> for Param {
            fn from(v: $t) -> Self {
                Self::Value(ParamData::Int(v as i64))
            }
        })*
    };
}

macro_rules! impl_from_float_for_param {
    ($($t:ty),*) => {
        $(impl From<$t> for Param {
            fn from(v: $t) -> Self {
                Self::Value(ParamData::Float(v as f64))
            }
        })*
    };
}
macro_rules! impl_from_for_param {
    ($(($t:ty, $variant:ident)),*) => {
        $(impl From<$t> for Param {
            fn from(v: $t) -> Self {
                Self::Value(ParamData::$variant(v))
            }
        })*
    };
}

impl_from_int_for_param!(i8, i16, i32, i64, u8, u16, u32, u64);
impl_from_float_for_param!(f32, f64);
impl_from_for_param!(
    (bool, Bool),
    (String, String),
    (Date, Date),
    (DateTime, DateTime),
    (Time, Time)
);
impl From<&str> for Param {
    fn from(value: &str) -> Self {
        Self::Value(ParamData::String(value.to_string()))
    }
}

pub fn lit(v: impl Into<Param>) -> Param {
    match v.into() {
        Param::Value(d) => Param::Literal(d),
        p => p,
    }
}

pub fn value(v: impl Into<Param>) -> Param {
    match v.into() {
        Param::Literal(d) => Param::Value(d),
        p => p,
    }
}
