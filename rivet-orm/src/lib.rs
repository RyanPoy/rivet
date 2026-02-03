pub mod schema;
pub mod sequel;

extern crate core;
#[cfg(test)]
extern crate self as rivet;

// 魔法：让 ::rivet 指向当前 crate 根部
#[cfg(test)]
pub use crate::schema::col as orm; // 建立 orm 别名，使 ::rivet::orm 等效于 crate::orm
