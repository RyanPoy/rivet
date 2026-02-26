#[derive(Clone, Copy, Debug)]
pub struct Op(&'static str);

impl AsRef<str> for Op {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub const ADD: Op = Op("+");
pub const SUB: Op = Op("-");
pub const MUL: Op = Op("*");
pub const DIV: Op = Op("/");
pub const MOD: Op = Op("%");

pub const EQ: Op = Op("=");
pub const NOT_EQ: Op = Op("<>");
pub const GT: Op = Op(">");
pub const GTE: Op = Op(">=");
pub const LT: Op = Op("<");
pub const LTE: Op = Op("<=");

pub const AND: Op = Op("AND");
pub const OR: Op = Op("OR");
pub const LIKE: Op = Op("LIKE");
pub const NOT_LIKE: Op = Op("NOT LIKE");
pub const IN: Op = Op("IN");
pub const NOT_IN: Op = Op("NOT IN");

pub const IS: Op = Op("IS");
pub const IS_NOT: Op = Op("IS NOT");
