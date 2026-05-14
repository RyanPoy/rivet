#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Char {
        max_length: Option<usize>,
    },
    Text,
    Int,
    BigInt,
    SmallInt,
    UnsignedInt,
    UnsignedSmallInt,
    Float,
    Double,
    Decimal {
        precision: Option<usize>,
        scale: Option<usize>,
    },
    Bool,
    Date,
    DateTime,
    Time,
    Json,
    Uuid,
    Blob,
    Auto,
    ForeignKey,
}

impl ColumnType {
    pub fn default_for_rust_type(rust_type: &str) -> Option<Self> {
        match rust_type {
            "i8" | "i16" | "i32" | "u8" | "u16" => Some(ColumnType::Int),
            "i64" | "u32" | "u64" => Some(ColumnType::BigInt),
            "f32" => Some(ColumnType::Float),
            "f64" => Some(ColumnType::Double),
            "bool" => Some(ColumnType::Bool),
            "String" | "&str" => Some(ColumnType::Char { max_length: None }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_type_default_for_rust_type() {
        assert_eq!(ColumnType::default_for_rust_type("i32").unwrap(), ColumnType::Int);
        assert_eq!(ColumnType::default_for_rust_type("i16").unwrap(), ColumnType::Int);
        assert_eq!(ColumnType::default_for_rust_type("i64").unwrap(), ColumnType::BigInt);
        assert_eq!(ColumnType::default_for_rust_type("u64").unwrap(), ColumnType::BigInt);
        assert_eq!(ColumnType::default_for_rust_type("f32").unwrap(), ColumnType::Float);
        assert_eq!(ColumnType::default_for_rust_type("f64").unwrap(), ColumnType::Double);
        assert_eq!(ColumnType::default_for_rust_type("bool").unwrap(), ColumnType::Bool);
        assert_eq!(
            ColumnType::default_for_rust_type("String").unwrap(),
            ColumnType::Char { max_length: None }
        );
        assert!(ColumnType::default_for_rust_type("UnknownType").is_none());
    }
}
